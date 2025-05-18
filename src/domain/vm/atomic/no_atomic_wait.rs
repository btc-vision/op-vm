//! -------------------------------------------------------------
//! # Atomic Wait Metering
//!
//! This module provides a metering middleware for the `atomic.wait`
//! and `memory.atomic.notify` instructions in WebAssembly.
//! It charges gas for the execution of these instructions
//! based on the time spent waiting.
//!
//! ## Usage
//! ```rust
//! use atomic_wait_metering::{AtomicWaitMetering, GasConfig};
//! let cfg = GasConfig { setup_gas: 5000, ..GasConfig::default() };
//! let mw  = AtomicWaitMetering::new(cfg, 1_000_000, |_| 0);
//! ```
//! ## Configuration
//! The `GasConfig` struct allows you to configure the gas costs
//! for the metering middleware.
//! The default values are:
//!
//! ```rust
//! let cfg = GasConfig {
//!    setup_gas: 2_000,
//!    ns_per_unit: 10_000, // 10 µs
//!    wait_fallback: 15_000,
//!    notify_gas: 3_000,
//! };
//! ```
//! -------------------------------------------------------------

#![cfg(feature = "contract-threading")]
#![allow(clippy::too_many_arguments)]

use crate::domain::runner::{MAX_NS, MAX_THREADS};
use std::sync::atomic::Ordering::{Relaxed, Release};
use std::{
    fmt::{Debug, Formatter},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};
use thiserror::Error;
use wasmer::wasmparser::BlockType;
use wasmer::{
    sys::{FunctionMiddleware, MiddlewareError, MiddlewareReaderState, ModuleMiddleware},
    wasmparser::Operator,
    LocalFunctionIndex, Type,
};
use wasmer_types::Mutability::Var;
use wasmer_types::{
    entity::EntityRef, ExportIndex, GlobalIndex, GlobalInit, GlobalType, ImportIndex, ModuleInfo,
    TableIndex, TableInitializer, TableType, TrapCode,
};
use Operator::*;

#[derive(Debug, Error)]
pub enum AtomicMeteringError {
    #[error("mutex poisoned")]
    Poison,
    #[error("globals not initialised")]
    GlobalsMissing,
    #[error("export `{0}` not found")]
    ExportMissing(&'static str),
    #[error("failed numeric conversion from/into wasmer value")]
    Conversion,
    #[error("failed to write to global")]
    SetFailed,
    #[error("atomic out of gas")]
    Exhausted,
}

impl From<AtomicMeteringError> for MiddlewareError {
    fn from(e: AtomicMeteringError) -> Self {
        MiddlewareError::new("AtomicWaitMetering", e.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GasConfig {
    #[allow(dead_code)]
    pub setup_gas: u64,
    pub wait_fallback: u64,
    pub notify_gas: u64,
    #[allow(dead_code)]
    pub spawn_cost: u64,
}

impl Default for GasConfig {
    fn default() -> Self {
        const SETUP: u64 = 2_000;
        const UNIT: u64 = 10_000;

        let max_units = (MAX_NS + UNIT - 1) / UNIT;

        Self {
            setup_gas: SETUP,
            wait_fallback: SETUP + max_units,
            notify_gas: 3_000,
            spawn_cost: 50_000,
        }
    }
}

fn ensure_table(m: &mut ModuleInfo) {
    if m.tables.is_empty() {
        m.tables.push(TableType::new(Type::FuncRef, 0, None));
    }
}

fn populate_table_slot(
    m: &mut ModuleInfo,
    sig_out: &AtomicU32,
    slot_out: &AtomicU32,
    module: &str,
    field: &str,
    slot: u32,
) {
    if let Some((_, ImportIndex::Function(fid))) = m
        .imports
        .iter()
        .find(|(k, _)| k.module == module && k.field == field)
    {
        let sig_idx = m.functions.get(*fid).unwrap();
        sig_out.store(sig_idx.index() as u32, Release);

        m.table_initializers.push(TableInitializer {
            table_index: TableIndex::new(0),
            base: None,
            offset: slot as usize,
            elements: vec![*fid].into_boxed_slice(),
        });
        slot_out.store(slot, Release);
    }
}

pub struct AtomicWaitMetering<F: Fn(&Operator) -> u64 + Send + Sync> {
    gas: GasConfig,
    initial_limit: u64,
    cost: Arc<F>,

    wait32_slot: AtomicU32,
    wait64_slot: AtomicU32,
    notify_slot: AtomicU32,
    spawn_slot: AtomicU32,

    wait32_sig: AtomicU32,
    wait64_sig: AtomicU32,
    notify_sig: AtomicU32,
    spawn_sig: AtomicU32,

    g_wait32: AtomicU32,
    g_wait64: AtomicU32,
    g_notify: AtomicU32,
    g_spawn: AtomicU32,

    globals: Arc<Mutex<Option<(GlobalIndex, GlobalIndex, GlobalIndex)>>>,
}

impl<F> Clone for AtomicWaitMetering<F>
where
    F: Fn(&Operator) -> u64 + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            gas: self.gas,
            initial_limit: self.initial_limit,
            cost: self.cost.clone(),

            wait32_slot: AtomicU32::new(self.wait32_slot.load(Relaxed)),
            wait64_slot: AtomicU32::new(self.wait64_slot.load(Relaxed)),
            notify_slot: AtomicU32::new(self.notify_slot.load(Relaxed)),
            spawn_slot: AtomicU32::new(self.spawn_slot.load(Relaxed)),

            wait32_sig: AtomicU32::new(self.wait32_sig.load(Relaxed)),
            wait64_sig: AtomicU32::new(self.wait64_sig.load(Relaxed)),
            notify_sig: AtomicU32::new(self.notify_sig.load(Relaxed)),
            spawn_sig: AtomicU32::new(self.spawn_sig.load(Relaxed)),

            g_wait32: AtomicU32::new(self.g_wait32.load(Relaxed)),
            g_wait64: AtomicU32::new(self.g_wait64.load(Relaxed)),
            g_notify: AtomicU32::new(self.g_notify.load(Relaxed)),
            g_spawn: AtomicU32::new(self.g_spawn.load(Relaxed)),

            globals: self.globals.clone(),
        }
    }
}

impl<F> AtomicWaitMetering<F>
where
    F: Fn(&Operator) -> u64 + Send + Sync,
{
    pub fn new(gas: GasConfig, initial_limit: u64, cost_fn: F) -> Self {
        Self {
            gas,
            initial_limit,
            cost: Arc::new(cost_fn),

            wait32_slot: AtomicU32::new(u32::MAX),
            wait64_slot: AtomicU32::new(u32::MAX),
            notify_slot: AtomicU32::new(u32::MAX),
            spawn_slot: AtomicU32::new(u32::MAX),

            wait32_sig: AtomicU32::new(u32::MAX),
            wait64_sig: AtomicU32::new(u32::MAX),
            notify_sig: AtomicU32::new(u32::MAX),
            spawn_sig: AtomicU32::new(u32::MAX),

            g_wait32: AtomicU32::new(u32::MAX),
            g_wait64: AtomicU32::new(u32::MAX),
            g_notify: AtomicU32::new(u32::MAX),
            g_spawn: AtomicU32::new(u32::MAX),

            globals: Arc::new(Mutex::new(None)),
        }
    }

    fn globals_safe(&self) -> (GlobalIndex, GlobalIndex, GlobalIndex) {
        match self.globals.lock() {
            Ok(g) => g.as_ref().copied().unwrap_or((
                GlobalIndex::new(0),
                GlobalIndex::new(0),
                GlobalIndex::new(0),
            )),
            Err(p) => p.into_inner().as_ref().copied().unwrap_or((
                GlobalIndex::new(0),
                GlobalIndex::new(0),
                GlobalIndex::new(0),
            )),
        }
    }
}

impl<F> Debug for AtomicWaitMetering<F>
where
    F: 'static + Fn(&Operator) -> u64 + Send + Sync,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AtomicWaitMetering")
            .field("gas", &self.gas)
            .field("initial_limit", &self.initial_limit)
            .field("globals", &self.globals_safe())
            .finish()
    }
}

impl<F> ModuleMiddleware for AtomicWaitMetering<F>
where
    F: Fn(&Operator) -> u64 + Send + Sync + 'static,
{
    fn generate_function_middleware(&self, _: LocalFunctionIndex) -> Box<dyn FunctionMiddleware> {
        use Ordering::Relaxed;
        let (rem, ex, thr) = self.globals_safe();
        Box::new(FunctionAtomicMetering {
            gas: self.gas,
            cost: self.cost.clone(),
            remaining: rem,
            exhausted: ex,
            _threads: thr,

            g_wait32: self.g_wait32.load(Relaxed),
            g_wait64: self.g_wait64.load(Relaxed),
            g_notify: self.g_notify.load(Relaxed),

            wait32_sig: self.wait32_sig.load(Relaxed),
            wait64_sig: self.wait64_sig.load(Relaxed),
            notify_sig: self.notify_sig.load(Relaxed),

            acc: 0,
        })
    }

    fn transform_module_info(&self, m: &mut ModuleInfo) -> Result<(), MiddlewareError> {
        let mut guard = self
            .globals
            .lock()
            .map_err(|_| AtomicMeteringError::Poison)?;
        if guard.is_some() {
            panic!("AtomicWaitMetering reused for several modules");
        }

        let _max_thr = m.globals.push(GlobalType::new(Type::I32, Var));
        m.global_initializers
            .push(GlobalInit::I32Const(MAX_THREADS));

        let g_remaining = m.globals.push(GlobalType::new(Type::I64, Var));
        m.global_initializers
            .push(GlobalInit::I64Const(self.initial_limit as i64));
        m.exports.insert(
            "atomic_metering_remaining".into(),
            ExportIndex::Global(g_remaining),
        );

        let g_exhausted = m.globals.push(GlobalType::new(Type::I32, Var));
        m.global_initializers.push(GlobalInit::I32Const(0));
        m.exports.insert(
            "atomic_metering_exhausted".into(),
            ExportIndex::Global(g_exhausted),
        );

        let g_threads = m.globals.push(GlobalType::new(Type::I32, Var));
        m.global_initializers.push(GlobalInit::I32Const(0));

        *guard = Some((g_remaining, g_exhausted, g_threads));

        let mut make_helper_global = |name: &'static str| -> GlobalIndex {
            let g = m.globals.push(GlobalType::new(Type::I32, Var));
            m.global_initializers.push(GlobalInit::I32Const(-1));
            m.exports.insert(name.into(), ExportIndex::Global(g));
            g
        };
        let gw32 = make_helper_global("g_atomic_wait32_idx");
        let gw64 = make_helper_global("g_atomic_wait64_idx");
        let gntf = make_helper_global("g_atomic_notify_idx");
        let gspn = make_helper_global("g_thread_spawn_idx");

        self.g_wait32.store(gw32.as_u32(), Release);
        self.g_wait64.store(gw64.as_u32(), Release);
        self.g_notify.store(gntf.as_u32(), Release);
        self.g_spawn.store(gspn.as_u32(), Release);

        ensure_table(m);
        let mut next_slot = 0u32;

        populate_table_slot(
            m,
            &self.wait32_sig,
            &self.wait32_slot,
            "env",
            "__atomic_wait32",
            {
                let s = next_slot;
                next_slot += 1;
                s
            },
        );

        populate_table_slot(
            m,
            &self.wait64_sig,
            &self.wait64_slot,
            "env",
            "__atomic_wait64",
            {
                let s = next_slot;
                next_slot += 1;
                s
            },
        );

        populate_table_slot(
            m,
            &self.notify_sig,
            &self.notify_slot,
            "env",
            "__atomic_notify",
            {
                let s = next_slot;
                next_slot += 1;
                s
            },
        );

        populate_table_slot(
            m,
            &self.spawn_sig,
            &self.spawn_slot,
            "env",
            "__thread_spawn",
            {
                let s = next_slot;
                //next_slot += 1;
                s
            },
        );

        Ok(())
    }
}

#[inline(always)]
fn flush_acc(
    state: &mut MiddlewareReaderState<'_>,
    remaining: GlobalIndex,
    exhausted: GlobalIndex,
    acc: &mut u64,
) {
    if *acc == 0 {
        return;
    }
    state.extend(&[
        GlobalGet {
            global_index: remaining.as_u32(),
        },
        I64Const { value: *acc as i64 },
        I64LtU,
        If {
            blockty: BlockType::Empty,
        },
        I32Const { value: 1 },
        GlobalSet {
            global_index: exhausted.as_u32(),
        },
        Unreachable,
        End,
        GlobalGet {
            global_index: remaining.as_u32(),
        },
        I64Const { value: *acc as i64 },
        I64Sub,
        GlobalSet {
            global_index: remaining.as_u32(),
        },
    ]);
    *acc = 0;
}

struct FunctionAtomicMetering<F: Fn(&Operator) -> u64 + Send + Sync> {
    gas: GasConfig,
    cost: Arc<F>,

    remaining: GlobalIndex,
    exhausted: GlobalIndex,
    _threads: GlobalIndex,

    g_wait32: u32,
    g_wait64: u32,
    g_notify: u32,

    wait32_sig: u32,
    wait64_sig: u32,
    notify_sig: u32,

    acc: u64,
}

impl<F> Debug for FunctionAtomicMetering<F>
where
    F: Fn(&Operator) -> u64 + Send + Sync,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionAtomicMetering").finish()
    }
}

impl<F> FunctionMiddleware for FunctionAtomicMetering<F>
where
    F: Fn(&Operator) -> u64 + Send + Sync,
{
    fn feed<'a>(
        &mut self,
        op: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        self.acc = self.acc.saturating_add((self.cost)(&op));

        match op {
            MemoryAtomicWait32 { .. } => {
                self.acc = self.acc.saturating_add(self.gas.wait_fallback);
                flush_acc(state, self.remaining, self.exhausted, &mut self.acc);

                state.extend(&[
                    GlobalGet {
                        global_index: self.g_wait32,
                    },
                    I32Const { value: -1 },
                    I32Eq,
                    If {
                        blockty: BlockType::Empty,
                    },
                    I32Const {
                        value: TrapCode::UnalignedAtomic as i32,
                    },
                    Unreachable,
                    End,
                    GlobalGet {
                        global_index: self.g_wait32,
                    },
                    CallIndirect {
                        type_index: self.wait32_sig,
                        table_index: 0,
                    },
                ]);
                return Ok(());
            }

            MemoryAtomicWait64 { .. } => {
                self.acc = self.acc.saturating_add(self.gas.wait_fallback);
                flush_acc(state, self.remaining, self.exhausted, &mut self.acc);

                state.extend(&[
                    GlobalGet {
                        global_index: self.g_wait64,
                    },
                    I32Const { value: -1 },
                    I32Eq,
                    If {
                        blockty: BlockType::Empty,
                    },
                    I32Const {
                        value: TrapCode::UnalignedAtomic as i32,
                    },
                    Unreachable,
                    End,
                    GlobalGet {
                        global_index: self.g_wait64,
                    },
                    CallIndirect {
                        type_index: self.wait64_sig,
                        table_index: 0,
                    },
                ]);
                return Ok(());
            }

            MemoryAtomicNotify { .. } => {
                self.acc = self.acc.saturating_add(self.gas.notify_gas);
                flush_acc(state, self.remaining, self.exhausted, &mut self.acc);

                state.extend(&[
                    GlobalGet {
                        global_index: self.g_notify,
                    },
                    I32Const { value: -1 },
                    I32Eq,
                    If {
                        blockty: BlockType::Empty,
                    },
                    I32Const {
                        value: TrapCode::UnalignedAtomic as i32,
                    },
                    Unreachable,
                    End,
                    GlobalGet {
                        global_index: self.g_notify,
                    },
                    CallIndirect {
                        type_index: self.notify_sig,
                        table_index: 0,
                    },
                ]);
                return Ok(());
            }

            _ => {}
        }

        state.push_operator(op);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicPoints {
    Remaining(u64),
    Exhausted,
}

pub fn get_points_atomic(
    store: &mut impl wasmer::AsStoreMut,
    inst: &wasmer::Instance,
) -> Result<AtomicPoints, AtomicMeteringError> {
    let exhausted_val: i32 = inst
        .exports
        .get_global("atomic_metering_exhausted")
        .map_err(|_| AtomicMeteringError::ExportMissing("atomic_metering_exhausted"))?
        .get(store)
        .try_into()
        .map_err(|_| AtomicMeteringError::Conversion)?;
    if exhausted_val != 0 {
        return Ok(AtomicPoints::Exhausted);
    }

    let remaining_val: u64 = inst
        .exports
        .get_global("atomic_metering_remaining")
        .map_err(|_| AtomicMeteringError::ExportMissing("atomic_metering_remaining"))?
        .get(store)
        .try_into()
        .map_err(|_| AtomicMeteringError::Conversion)?;
    Ok(AtomicPoints::Remaining(remaining_val))
}

#[allow(dead_code)]
pub fn set_points_atomic(
    store: &mut impl wasmer::AsStoreMut,
    inst: &wasmer::Instance,
    pts: u64,
) -> Result<(), AtomicMeteringError> {
    inst.exports
        .get_global("atomic_metering_remaining")
        .map_err(|_| AtomicMeteringError::ExportMissing("atomic_metering_remaining"))?
        .set(store, (pts as i64).into())
        .map_err(|_| AtomicMeteringError::SetFailed)?;

    inst.exports
        .get_global("atomic_metering_exhausted")
        .map_err(|_| AtomicMeteringError::ExportMissing("atomic_metering_exhausted"))?
        .set(store, 0_i32.into())
        .map_err(|_| AtomicMeteringError::SetFailed)?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_total_threads(
    store: &mut impl wasmer::AsStoreMut,
    inst: &wasmer::Instance,
) -> Result<u32, AtomicMeteringError> {
    let threads_val: i32 = inst
        .exports
        .get_global("g_thread_spawn_idx")
        .map_err(|_| AtomicMeteringError::ExportMissing("g_thread_spawn_idx"))?
        .get(store)
        .try_into()
        .map_err(|_| AtomicMeteringError::Conversion)?;

    Ok(threads_val as u32)
}

#[inline]
pub fn default_cost_atomic(op: &Operator) -> u64 {
    if let MemoryAtomicNotify { .. } = op {
        3_000
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicHelper {
    Wait32,
    Wait64,
    Notify,
    Spawn,
}

pub fn set_helper_index_atomic(
    store: &mut impl wasmer::AsStoreMut,
    inst: &wasmer::Instance,
    helper: AtomicHelper,
    table_slot: u32,
) -> Result<(), AtomicMeteringError> {
    let global_name = match helper {
        AtomicHelper::Wait32 => "g_atomic_wait32_idx",
        AtomicHelper::Wait64 => "g_atomic_wait64_idx",
        AtomicHelper::Notify => "g_atomic_notify_idx",
        AtomicHelper::Spawn => "g_thread_spawn_idx",
    };

    inst.exports
        .get_global(global_name)
        .map_err(|_| AtomicMeteringError::ExportMissing(global_name))?
        .set(store, (table_slot as i32).into())
        .map_err(|_| AtomicMeteringError::SetFailed)
}

#[cfg(test)]
mod tests {
    use crate::domain::vm::{
        default_cost_atomic, get_points_atomic, set_helper_index_atomic, set_points_atomic,
        AtomicHelper, AtomicPoints, AtomicWaitMetering, ClampBulkMem, GasConfig,
    };
    use std::sync::Arc;
    use wasmer::{
        imports,
        sys::{CompilerConfig, Cranelift, EngineBuilder},
        wat2wasm, Function, Instance, Module, Store, TypedFunction,
    };

    fn stub_wait32(_addr: i32, _expect: i32, _timeout: i64) -> i32 {
        0
    }
    fn stub_wait64(_addr: i32, _expect_hi: i64, _timeout: i64) -> i32 {
        0
    }
    fn stub_notify(_addr: i32, _count: i32) -> i32 {
        0
    }
    fn stub_spawn() -> i32 {
        0
    }

    /// Minimal WAT snippets
    /// Each module imports **all** helper stubs (because the middleware always
    /// inserts them) but exercises only the specific one we care about.

    const WAT_WAIT32: &str = r#"
(module
  (import "env" "__atomic_wait32"  (func $w32 (param i32 i32 i64) (result i32)))
  (import "env" "__atomic_wait64"  (func (param i32 i64 i64) (result i32)))
  (import "env" "__atomic_notify"  (func (param i32 i32)     (result i32)))
  (import "env" "__thread_spawn"   (func (result i32)))

  (func (export "call")
    i32.const 0
    i32.const 0
    i64.const 0
    call $w32
    drop))
"#;

    const WAT_NOTIFY: &str = r#"
(module
  (import "env" "__atomic_wait32"  (func (param i32 i32 i64) (result i32)))
  (import "env" "__atomic_wait64"  (func (param i32 i64 i64) (result i32)))
  (import "env" "__atomic_notify"  (func $ntf (param i32 i32) (result i32)))
  (import "env" "__thread_spawn"   (func (result i32)))

  (func (export "call")
    i32.const 0
    i32.const 1
    call $ntf
    drop))
"#;

    const WAT_SPAWN: &str = r#"
(module
  (import "env" "__atomic_wait32"  (func (param i32 i32 i64) (result i32)))
  (import "env" "__atomic_wait64"  (func (param i32 i64 i64) (result i32)))
  (import "env" "__atomic_notify"  (func (param i32 i32)     (result i32)))
  (import "env" "__thread_spawn"   (func $spawn (result i32)))

  (func (export "call")
    call $spawn
    drop))
"#;

    /// Helper to build a store+module+instance with the middleware installed
    fn instantiate(wat_src: &str, points: u64) -> (Store, Instance, GasConfig) {
        let middleware = Arc::new(AtomicWaitMetering::new(
            GasConfig::default(),
            points,
            default_cost_atomic,
        ));

        let mut compiler = Cranelift::default();
        compiler.push_middleware(middleware);
        compiler.push_middleware(ClampBulkMem::new(1 << 20)); // 1 MiB copy cap

        let mut store = Store::new(EngineBuilder::new(compiler));
        let wasm = wat2wasm(wat_src.as_ref()).unwrap();
        let module = Module::new(&store, wasm).unwrap();

        let import_obj = imports! {
            "env" => {
                "__atomic_wait32"  => Function::new_typed(&mut store, stub_wait32),
                "__atomic_wait64"  => Function::new_typed(&mut store, stub_wait64),
                "__atomic_notify"  => Function::new_typed(&mut store, stub_notify),
                "__thread_spawn"   => Function::new_typed(&mut store, stub_spawn),
            }
        };

        let instance = Instance::new(&mut store, &module, &import_obj).unwrap();

        for (idx, imp) in instance.module().imports().enumerate() {
            match (imp.module(), imp.name()) {
                ("env", "__atomic_wait32") => {
                    set_helper_index_atomic(&mut store, &instance, AtomicHelper::Wait32, idx as u32)
                        .unwrap()
                }
                ("env", "__atomic_wait64") => {
                    set_helper_index_atomic(&mut store, &instance, AtomicHelper::Wait64, idx as u32)
                        .unwrap()
                }
                ("env", "__atomic_notify") => {
                    set_helper_index_atomic(&mut store, &instance, AtomicHelper::Notify, idx as u32)
                        .unwrap()
                }
                ("env", "__thread_spawn") => {
                    set_helper_index_atomic(&mut store, &instance, AtomicHelper::Spawn, idx as u32)
                        .unwrap()
                }
                _ => {}
            }
        }

        (store, instance, GasConfig::default())
    }

    #[test]
    fn wait32_consumes_expected_gas() {
        const LIMIT: u64 = 3_000; // give ourselves plenty

        let (mut store, instance, gas_cfg) = instantiate(WAT_WAIT32, LIMIT);
        let call: TypedFunction<(), ()> = instance
            .exports
            .get_function("call")
            .unwrap()
            .typed(&store)
            .unwrap();

        // remaining before any call
        assert_eq!(
            get_points_atomic(&mut store, &instance).unwrap(),
            AtomicPoints::Remaining(LIMIT)
        );

        // first call succeeds
        call.call(&mut store).unwrap();

        let expect_after = LIMIT - gas_cfg.wait_fallback;
        assert_eq!(
            get_points_atomic(&mut store, &instance).unwrap(),
            AtomicPoints::Remaining(expect_after)
        );

        // second call should exhaust the meter
        let res = call.call(&mut store);
        assert!(res.is_err(), "second call must trap (out of gas)");
        assert_eq!(
            get_points_atomic(&mut store, &instance).unwrap(),
            AtomicPoints::Exhausted
        );
    }

    #[test]
    fn notify_costs_notify_gas_only() {
        const LIMIT: u64 = 10_000;

        let (mut store, instance, gas_cfg) = instantiate(WAT_NOTIFY, LIMIT);
        let call: TypedFunction<(), ()> = instance
            .exports
            .get_function("call")
            .unwrap()
            .typed(&store)
            .unwrap();

        call.call(&mut store).unwrap();

        let remaining = match get_points_atomic(&mut store, &instance) {
            Ok(AtomicPoints::Remaining(r)) => r,
            _ => panic!("unexpected exhaustion"),
        };

        assert_eq!(remaining, LIMIT - gas_cfg.notify_gas);
    }

    #[test]
    fn set_points_resets_exhausted_flag() {
        const LIMIT_FIRST: u64 = 1_500; // < wait_fallback → first call traps
        const LIMIT_SECOND: u64 = 3_000; // > wait_fallback → second call runs

        let (mut store, instance, _) = instantiate(WAT_WAIT32, LIMIT_FIRST);
        let call: TypedFunction<(), ()> = instance
            .exports
            .get_function("call")
            .unwrap()
            .typed(&store)
            .unwrap();

        // first call exhausts and traps
        assert!(call.call(&mut store).is_err());
        assert_eq!(
            get_points_atomic(&mut store, &instance).unwrap(),
            AtomicPoints::Exhausted
        );

        // reset meter to a larger allowance
        set_points_atomic(&mut store, &instance, LIMIT_SECOND).unwrap();
        assert_eq!(
            get_points_atomic(&mut store, &instance).unwrap(),
            AtomicPoints::Remaining(LIMIT_SECOND)
        );

        // now the call succeeds
        call.call(&mut store).unwrap();
    }

    #[test]
    fn thread_spawn_charges_and_updates_counter() {
        const LIMIT: u64 = 100_000;

        let (mut store, instance, gas_cfg) = instantiate(WAT_SPAWN, LIMIT);
        let call: TypedFunction<(), ()> = instance
            .exports
            .get_function("call")
            .unwrap()
            .typed(&store)
            .unwrap();

        call.call(&mut store).unwrap();

        let remaining = match get_points_atomic(&mut store, &instance) {
            Ok(AtomicPoints::Remaining(r)) => r,
            _ => panic!("exhausted unexpectedly"),
        };

        assert_eq!(remaining, LIMIT - gas_cfg.spawn_cost);
    }
}
