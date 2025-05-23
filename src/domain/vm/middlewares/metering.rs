//! `metering` is a middleware for tracking how many operators are
//! executed in total and putting a limit on the total number of
//! operators executed. The WebAssembly instance execution is stopped
//! when the limit is reached.
//!
//! # Example
//!
//! [See the `metering` detailed and complete
//! example](https://github.com/wasmerio/wasmer/blob/main/examples/metering.rs).

use crate::domain::vm::{
    MEMORY_COPY_BASE, MEMORY_COPY_PER_BLOCK, MEMORY_FILL_BASE, MEMORY_FILL_PER_BLOCK,
    TABLE_COPY_BASE, TABLE_COPY_PER_ELEM, TABLE_FILL_BASE, TABLE_FILL_PER_ELEM, TABLE_GROW_BASE,
    TABLE_GROW_PER_ELEM,
};
use std::convert::TryInto;
use std::fmt;
use std::sync::{Arc, Mutex};
use wasmer::wasmparser::Operator::*;
use wasmer::wasmparser::{BlockType as WpTypeOrFuncType, Operator};
use wasmer::{
    sys::{FunctionMiddleware, MiddlewareError, MiddlewareReaderState, ModuleMiddleware},
    AsStoreMut, ExportIndex, GlobalInit, GlobalType, Instance, LocalFunctionIndex, Mutability,
    Type,
};
use wasmer_types::{GlobalIndex, ModuleInfo};

const MAX_U64_COST: u64 = u64::MAX;
pub const MAX_ACCUM: u64 = MAX_U64_COST - 1;
//const FLUSH_THRESHOLD: u64 = 8;

#[derive(Clone)]
struct MeteringGlobalIndexes {
    remaining: GlobalIndex,
    exhausted: GlobalIndex,
    scratch_len: GlobalIndex,
}

impl MeteringGlobalIndexes {
    /// The global index in the current module for remaining points.
    fn remaining_points(&self) -> GlobalIndex {
        self.remaining
    }

    /// The global index in the current module for a boolean indicating whether points are exhausted
    /// or not.
    /// This boolean is represented as a i32 global:
    ///   * 0: there are remaining points
    ///   * 1: points have been exhausted
    fn points_exhausted(&self) -> GlobalIndex {
        self.exhausted
    }
}

impl fmt::Debug for MeteringGlobalIndexes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeteringGlobalIndexes")
            .field("remaining_points", &self.remaining_points())
            .field("points_exhausted", &self.points_exhausted())
            .finish()
    }
}

/// The module-level metering middleware.
///
/// # Panic
///
/// An instance of `Metering` should _not_ be shared among different
/// modules, since it tracks module-specific information like the
/// global index to store metering state. Attempts to use a `Metering`
/// instance from multiple modules will result in a panic.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use wasmer::{wasmparser::Operator, sys::CompilerConfig};
/// use wasmer_middlewares::Metering;
///
/// fn create_metering_middleware(compiler_config: &mut dyn CompilerConfig) {
///     // Let's define a dummy cost function,
///     // which counts 1 for all operators.
///     let cost_function = |_operator: &Operator| -> u64 { 1 };
///
///     // Let's define the initial limit.
///     let initial_limit = 10;
///
///     // Let's creating the metering middleware.
///     let metering = Arc::new(Metering::new(
///         initial_limit,
///         cost_function
///     ));
///
///     // Finally, let's push the middleware.
///     compiler_config.push_middleware(metering);
/// }
/// ```
pub struct Metering<F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync> {
    /// Initial limit of points.
    initial_limit: u64,

    /// Function that maps each operator to a cost in "points".
    cost_function: Arc<F>,

    /// The global indexes for metering points.
    global_indexes: Mutex<Option<MeteringGlobalIndexes>>,

    /// Vector holding (params, results) for each type index.
    type_arities: Mutex<Option<Arc<Vec<(u32, u32)>>>>,

    clamp_max_len: u32,
}

/// The function-level metering middleware.
pub struct FunctionMetering<F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync> {
    /// Function that maps each operator to a cost in "points".
    cost_function: Arc<F>,

    /// The global indexes for metering points.
    global_indexes: MeteringGlobalIndexes,

    /// Vector holding (params, results) for each type index.
    type_arities: Arc<Vec<(u32, u32)>>,

    /// Accumulated cost of the current basic block.
    accumulated_cost: u64,

    last_i32_const: Option<u32>,

    max_len: u32,
}

/// Represents the type of the metering points, either `Remaining` or
/// `Exhausted`.
///
/// # Example
///
/// See the [`get_remaining_points`] function to get an example.
#[derive(Debug, Eq, PartialEq)]
pub enum MeteringPoints {
    /// The given number of metering points is left for the execution.
    /// If the value is 0, all points are consumed but the execution
    /// was not terminated.
    Remaining(u64),

    /// The execution was terminated because the metering points were
    /// exhausted.  You can recover from this state by setting the
    /// points via [`set_remaining_points`] and restart the execution.
    Exhausted,
}

impl<F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync> Metering<F> {
    /// Creates a `Metering` middleware.
    ///
    /// When providing a cost function, you should consider that branching operations do
    /// additional work to track the metering points and probably need to have a higher cost.
    /// To find out which operations are affected by this, you can call [`is_accounting`].
    pub fn new(initial_limit: u64, cost_function: F, clamp_max_len: u32) -> Self {
        Self {
            initial_limit,
            cost_function: Arc::new(cost_function),
            global_indexes: Mutex::new(None),
            type_arities: Mutex::new(None),
            clamp_max_len,
        }
    }
}

impl<F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync> fmt::Debug for Metering<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Metering")
            .field("initial_limit", &self.initial_limit)
            .field("cost_function", &"<function>")
            .field("global_indexes", &self.global_indexes)
            .field("type_arities", &self.type_arities)
            .field("clamp_max_len", &self.clamp_max_len)
            .finish()
    }
}

impl<F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync + 'static> ModuleMiddleware
    for Metering<F>
{
    /// Generates a `FunctionMiddleware` for a given function.
    fn generate_function_middleware(&self, _: LocalFunctionIndex) -> Box<dyn FunctionMiddleware> {
        Box::new(FunctionMetering {
            cost_function: self.cost_function.clone(),
            global_indexes: self.global_indexes.lock().unwrap().clone().unwrap(),
            accumulated_cost: 0,
            type_arities: self.type_arities.lock().unwrap().clone().unwrap(),
            last_i32_const: None,
            max_len: self.clamp_max_len,
        })
    }

    /// Transforms a `ModuleInfo` struct in-place. This is called before application on functions begins.
    fn transform_module_info(&self, module_info: &mut ModuleInfo) -> Result<(), MiddlewareError> {
        let mut global_indexes = self.global_indexes.lock().unwrap();

        if global_indexes.is_some() {
            panic!("Metering::transform_module_info: Attempting to use a `Metering` middleware from multiple modules.");
        }

        // Append a global for remaining points and initialize it.
        let remaining_points_global_index = module_info
            .globals
            .push(GlobalType::new(Type::I64, Mutability::Var));

        module_info
            .global_initializers
            .push(GlobalInit::I64Const(self.initial_limit as i64));

        module_info.exports.insert(
            "wasmer_metering_remaining_points".to_string(),
            ExportIndex::Global(remaining_points_global_index),
        );

        // Append a global for the exhausted points boolean and initialize it.
        let points_exhausted_global_index = module_info
            .globals
            .push(GlobalType::new(Type::I32, Mutability::Var));

        module_info
            .global_initializers
            .push(GlobalInit::I32Const(0));

        module_info.exports.insert(
            "wasmer_metering_points_exhausted".to_string(),
            ExportIndex::Global(points_exhausted_global_index),
        );

        let scratch_len = module_info
            .globals
            .push(GlobalType::new(Type::I32, Mutability::Var));

        module_info
            .global_initializers
            .push(GlobalInit::I32Const(0));

        *global_indexes = Some(MeteringGlobalIndexes {
            remaining: remaining_points_global_index,
            exhausted: points_exhausted_global_index,
            scratch_len,
        });

        // collect (params, results) arity for each type index
        let mut arities: Vec<(u32, u32)> = Vec::new();
        for sig in module_info.signatures.values() {
            arities.push((sig.params().len() as u32, sig.results().len() as u32));
        }

        *self.type_arities.lock().unwrap() = Some(Arc::new(arities));

        Ok(())
    }
}

#[inline(always)]
fn is_trap_arith(op: &Operator) -> bool {
    matches!(
        op,
        I32DivU { .. }
            | I32DivS { .. }
            | I64DivU { .. }
            | I64DivS { .. }
            | I32RemU { .. }
            | I32RemS { .. }
            | I64RemU { .. }
            | I64RemS { .. }
    )
}

/// Returns `true` if and only if the given operator is an accounting operator.
/// Accounting operators do additional work to track the metering points.
pub fn is_accounting(operator: &Operator) -> bool {
    // Possible sources and targets of a branch.
    matches!(
        operator,
        Loop { .. } // loop headers are branch targets
            | End // block ends are branch targets
            | If { .. } // branch source, "if" can branch to else branch
            | Else // "else" is the "end" of an if branch
            | Br { .. } // branch source
            | BrTable { .. } // branch source
            | BrIf { .. } // branch source
            | Call { .. } // function call - branch source
            | CallIndirect { .. } // function call - branch source
            | Return // end of function - branch source
            // exceptions proposal
            | Throw { .. } // branch source
            | ThrowRef // branch source
            | Rethrow { .. } // branch source
            | Delegate { .. } // branch source
            | Catch { .. } // branch target
            // tail_call proposal
            | ReturnCall { .. } // branch source
            | ReturnCallIndirect { .. } // branch source
            // gc proposal
            | BrOnCast { .. } // branch source
            | BrOnCastFail { .. } // branch source
            // function_references proposal
            | CallRef { .. } // branch source
            | ReturnCallRef { .. } // branch source
            | BrOnNull { .. } // branch source
            | BrOnNonNull { .. }

            // arithmetic traps
            /*| I32DivU { .. } | I32DivS { .. }
            | I64DivU { .. } | I64DivS { .. }
            | I32RemU { .. } | I32RemS { .. }
            | I64RemU { .. } | I64RemS { .. }
            | I32TruncF32S { .. } | I32TruncF32U { .. }
            | I32TruncF64S { .. } | I32TruncF64U { .. }
            | I64TruncF32S { .. } | I64TruncF32U { .. }
            | I64TruncF64S { .. } | I64TruncF64U { .. }

            // memory traps
            | I32Load { .. } | I64Load { .. } | F32Load { .. } | F64Load { .. }
            | I32Load8S { .. } | I32Load8U { .. } | I32Load16S { .. } | I32Load16U { .. }
            | I64Load8S { .. } | I64Load8U { .. } | I64Load16S { .. } | I64Load16U { .. }
            | I64Load32S { .. } | I64Load32U { .. }
            | I32Store { .. } | I64Store { .. } | F32Store { .. } | F64Store { .. }
            | I32Store8 { .. } | I32Store16 { .. }
            | I64Store8 { .. } | I64Store16 { .. } | I64Store32 { .. }
            | I32AtomicLoad { .. } | I64AtomicLoad { .. }
            | I32AtomicStore { .. } | I64AtomicStore { .. }
            | I32AtomicRmwAdd { .. } | I64AtomicRmwAdd { .. }

            // table traps
            | TableGet { .. } | TableSet { .. } | TableGrow { .. }
            | TableFill { .. } | TableCopy { .. } | TableInit { .. }*/

            // Always-trap op
            | Unreachable
    )
}

impl<F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync> fmt::Debug for FunctionMetering<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMetering")
            .field("cost_function", &"<function>")
            .field("global_indexes", &self.global_indexes)
            .finish()
    }
}

impl<F> FunctionMetering<F>
where
    F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync,
{
    /// flush accumulated gas to the global counter
    #[inline(always)]
    fn flush_accumulated_cost(&mut self, state: &mut MiddlewareReaderState<'_>) {
        if self.accumulated_cost == 0 {
            return;
        }
        let g = &self.global_indexes;
        state.extend(&[
            GlobalGet {
                global_index: g.remaining.as_u32(),
            },
            I64Const {
                value: self.accumulated_cost as i64,
            },
            I64LtU,
            If {
                blockty: WpTypeOrFuncType::Empty,
            },
            I32Const { value: 1 },
            GlobalSet {
                global_index: g.exhausted.as_u32(),
            },
            Unreachable,
            End,
            GlobalGet {
                global_index: g.remaining.as_u32(),
            },
            I64Const {
                value: self.accumulated_cost as i64,
            },
            I64Sub,
            GlobalSet {
                global_index: g.remaining.as_u32(),
            },
        ]);
        self.accumulated_cost = 0;
    }

    #[cfg(feature = "debug-metering")]
    #[inline(always)]
    fn log_len_based(&mut self, op: &'static str, per_block: u64, block: u32) {
        if let Some(len) = self.last_i32_const.take() {
            let blocks = (len as u64 + block as u64 - 1) / block as u64;
            let charged = per_block * blocks;
            log::info!(
                "instrumenting {}: constant len = {} → +{} gas",
                op,
                len,
                charged
            );
        } else {
            log::info!(
                "instrumenting {}: dynamic len → +{}·blocks gas",
                op,
                per_block
            );
        }
    }

    #[inline(always)]
    fn clamp_len(&self, state: &mut MiddlewareReaderState<'_>) {
        state.extend(&[
            GlobalGet {
                global_index: self.global_indexes.scratch_len.as_u32(),
            },
            I32Const {
                value: self.max_len as i32,
            },
            I32GtU,
            If {
                blockty: WpTypeOrFuncType::Empty,
            },
            Unreachable,
            End,
        ]);
    }

    /// Injects `global.scratch_len`-based gas accounting.
    #[inline(always)]
    fn charge_len_based(
        &self,
        state: &mut MiddlewareReaderState<'_>,
        base: u64,
        per_block: u64,
        log2_block: i32,
    ) {
        let g = &self.global_indexes;

        state.extend(&[
            GlobalGet {
                global_index: g.scratch_len.as_u32(),
            },
            I32Const {
                value: (1 << log2_block) - 1,
            },
            I32Add,
            I32Const { value: log2_block },
            I32ShrU,
            I64ExtendI32U,
            I64Const {
                value: per_block as i64,
            },
            I64Mul, // variable part
            I64Const { value: base as i64 },
            I64Add, // total cost
            GlobalGet {
                global_index: g.remaining.as_u32(),
            },
            I64GtU,
            If {
                blockty: WpTypeOrFuncType::Empty,
            },
            I32Const { value: 1 },
            GlobalSet {
                global_index: g.exhausted.as_u32(),
            },
            Unreachable,
            End,
            // remaining -= cost
            GlobalGet {
                global_index: g.remaining.as_u32(),
            },
            I64Const { value: base as i64 },
            GlobalGet {
                global_index: g.scratch_len.as_u32(),
            },
            I32Const {
                value: (1 << log2_block) - 1,
            },
            I32Add,
            I32Const { value: log2_block },
            I32ShrU,
            I64ExtendI32U,
            I64Const {
                value: per_block as i64,
            },
            I64Mul,
            I64Add,
            I64Sub,
            GlobalSet {
                global_index: g.remaining.as_u32(),
            },
        ]);

        // push len back for the real instruction
        state.push_operator(GlobalGet {
            global_index: g.scratch_len.as_u32(),
        });
    }
}

/// log2 of the block size for each metered op
const LOG2_BYTE_BLOCK: i32 = 4; // 2⁴ = 16 bytes
const LOG2_ELEM_BLOCK: i32 = 0; // 1 element

impl<F> FunctionMiddleware for FunctionMetering<F>
where
    F: Fn(&Operator, Option<(u32, u32)>) -> u64 + Send + Sync,
{
    fn feed<'a>(
        &mut self,
        operator: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        use Operator::*;

        if let I32Const { value } = operator {
            self.last_i32_const = Some(value as u32);
        }

        match operator {
            MemoryFill { .. } => {
                state.extend(&[GlobalSet {
                    global_index: self.global_indexes.scratch_len.as_u32(),
                }]);
                #[cfg(feature = "debug-metering")]
                self.log_len_based("memory.fill", MEMORY_FILL_PER_BLOCK, 16);
                self.clamp_len(state);
                self.charge_len_based(
                    state,
                    MEMORY_FILL_BASE,
                    MEMORY_FILL_PER_BLOCK,
                    LOG2_BYTE_BLOCK,
                );
            }

            MemoryCopy { .. } => {
                // | MemoryInit { .. } add it if we ever enable it.
                state.extend(&[GlobalSet {
                    global_index: self.global_indexes.scratch_len.as_u32(),
                }]);
                #[cfg(feature = "debug-metering")]
                self.log_len_based("memory.copy/init", MEMORY_COPY_PER_BLOCK, 16);
                self.clamp_len(state);
                self.charge_len_based(
                    state,
                    MEMORY_COPY_BASE,
                    MEMORY_COPY_PER_BLOCK,
                    LOG2_BYTE_BLOCK,
                );
            }

            #[cfg(feature = "table-metering")]
            TableCopy { .. } | TableInit { .. } => {
                state.extend(&[GlobalSet {
                    global_index: self.global_indexes.scratch_len.as_u32(),
                }]);
                #[cfg(feature = "debug-metering")]
                self.log_len_based("table.copy/init", TABLE_COPY_PER_ELEM, 1);
                self.clamp_len(state);
                self.charge_len_based(state, TABLE_COPY_BASE, TABLE_COPY_PER_ELEM, LOG2_ELEM_BLOCK);
            }

            #[cfg(feature = "table-metering")]
            TableFill { .. } => {
                state.extend(&[GlobalSet {
                    global_index: self.global_indexes.scratch_len.as_u32(),
                }]);
                #[cfg(feature = "debug-metering")]
                self.log_len_based("table.fill", TABLE_FILL_PER_ELEM, 1);
                self.clamp_len(state);
                self.charge_len_based(state, TABLE_FILL_BASE, TABLE_FILL_PER_ELEM, LOG2_ELEM_BLOCK);
            }

            #[cfg(feature = "table-metering")]
            TableGrow { .. } => {
                state.extend(&[GlobalSet {
                    global_index: self.global_indexes.scratch_len.as_u32(),
                }]);
                #[cfg(feature = "debug-metering")]
                self.log_len_based("table.grow", TABLE_GROW_PER_ELEM, 1);
                self.clamp_len(state);
                self.charge_len_based(state, TABLE_GROW_BASE, TABLE_GROW_PER_ELEM, LOG2_ELEM_BLOCK);
            }

            _ => {}
        }

        let arity = match &operator {
            CallIndirect { type_index, .. } => self.type_arities.get(*type_index as usize).copied(),
            _ => None,
        };

        let cost = (self.cost_function)(&operator, arity).min(MAX_ACCUM);
        self.accumulated_cost = self.accumulated_cost.saturating_add(cost);

        // micro-batch flush
        if is_accounting(&operator) && self.accumulated_cost > 0 || is_trap_arith(&operator)
        //self.accumulated_cost >= FLUSH_THRESHOLD
        {
            self.flush_accumulated_cost(state);
        }

        /*if is_accounting(&operator) && self.accumulated_cost > 0 {
            let g = &self.global_indexes;
            state.extend(&[
                GlobalGet {
                    global_index: g.remaining.as_u32(),
                },
                I64Const {
                    value: self.accumulated_cost as i64,
                },
                I64LtU,
                If {
                    blockty: WpTypeOrFuncType::Empty,
                },
                I32Const { value: 1 },
                GlobalSet {
                    global_index: g.exhausted.as_u32(),
                },
                Unreachable,
                End,
                GlobalGet {
                    global_index: g.remaining.as_u32(),
                },
                I64Const {
                    value: self.accumulated_cost as i64,
                },
                I64Sub,
                GlobalSet {
                    global_index: g.remaining.as_u32(),
                },
            ]);
            self.accumulated_cost = 0;
        }*/

        state.push_operator(operator);
        Ok(())
    }
}

/// Get the remaining points in an [`Instance`][wasmer::Instance].
///
/// Note: This can be used in a headless engine after an ahead-of-time
/// compilation as all required state lives in the instance.
///
/// # Panic
///
/// The [`Instance`][wasmer::Instance) must have been processed with
/// the [`Metering`] middleware at compile time, otherwise this will
/// panic.
///
/// # Example
///
/// ```rust
/// use wasmer::Instance;
/// use wasmer::AsStoreMut;
/// use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
///
/// /// Check whether the instance can continue to run based on the
/// /// number of remaining points.
/// fn can_continue_to_run(store: &mut impl AsStoreMut, instance: &Instance) -> bool {
///     matches!(get_remaining_points(store, instance), MeteringPoints::Remaining(points) if points > 0)
/// }
/// ```
pub fn get_remaining_points(ctx: &mut impl AsStoreMut, instance: &Instance) -> MeteringPoints {
    let exhausted: i32 = instance
        .exports
        .get_global("wasmer_metering_points_exhausted")
        .expect("Can't get `wasmer_metering_points_exhausted` from Instance")
        .get(ctx)
        .try_into()
        .expect("`wasmer_metering_points_exhausted` from Instance has wrong type");

    if exhausted > 0 {
        return MeteringPoints::Exhausted;
    }

    let points = instance
        .exports
        .get_global("wasmer_metering_remaining_points")
        .expect("Can't get `wasmer_metering_remaining_points` from Instance")
        .get(ctx)
        .try_into()
        .expect("`wasmer_metering_remaining_points` from Instance has wrong type");

    MeteringPoints::Remaining(points)
}

/// Set the new provided remaining points in an
/// [`Instance`][wasmer::Instance].
///
/// Note: This can be used in a headless engine after an ahead-of-time
/// compilation as all required state lives in the instance.
///
/// # Panic
///
/// The given [`Instance`][wasmer::Instance] must have been processed
/// with the [`Metering`] middleware at compile time, otherwise this
/// will panic.
///
/// # Example
///
/// ```rust
/// use wasmer::{AsStoreMut, Instance};
/// use wasmer_middlewares::metering::set_remaining_points;
///
/// fn update_remaining_points(store: &mut impl AsStoreMut, instance: &Instance) {
///     // The new limit.
///     let new_limit = 10;
///
///     // Update the remaining points to the `new_limit`.
///     set_remaining_points(store, instance, new_limit);
/// }
/// ```
pub fn set_remaining_points(ctx: &mut impl AsStoreMut, instance: &Instance, points: u64) {
    instance
        .exports
        .get_global("wasmer_metering_remaining_points")
        .expect("Can't get `wasmer_metering_remaining_points` from Instance")
        .set(ctx, points.into())
        .expect("Can't set `wasmer_metering_remaining_points` in Instance");

    instance
        .exports
        .get_global("wasmer_metering_points_exhausted")
        .expect("Can't get `wasmer_metering_points_exhausted` from Instance")
        .set(ctx, 0i32.into())
        .expect("Can't set `wasmer_metering_points_exhausted` in Instance");
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::runner::MAX_MEMORY_COPY_SIZE;
    use std::sync::Arc;
    use wasmer::sys::EngineBuilder;
    use wasmer::{
        imports,
        sys::{CompilerConfig, Cranelift},
        wat2wasm, Module, Store, TypedFunction,
    };

    fn cost_function(operator: &Operator, _: Option<(u32, u32)>) -> u64 {
        match operator {
            LocalGet { .. } | I32Const { .. } => 1,
            I32Add { .. } => 2,
            _ => 0,
        }
    }

    fn bytecode() -> Vec<u8> {
        wat2wasm(
            br#"(module
            (type $add_t (func (param i32) (result i32)))
            (func $add_one_f (type $add_t) (param $value i32) (result i32)
                local.get $value
                i32.const 1
                i32.add)
            (func $short_loop_f
                (local $x f64) (local $j i32)
                (local.set $x (f64.const 5.5))

                (loop $named_loop
                    ;; $j++
                    local.get $j
                    i32.const 1
                    i32.add
                    local.set $j

                    ;; if $j < 5, one more time
                    local.get $j
                    i32.const 5
                    i32.lt_s
                    br_if $named_loop
                )
            )
            (func $infi_loop_f
                (loop $infi_loop_start
                    br $infi_loop_start
                )
            )
            (export "add_one" (func $add_one_f))
            (export "short_loop" (func $short_loop_f))
            (export "infi_loop" (func $infi_loop_f))
        )"#,
        )
        .unwrap()
        .into()
    }

    #[test]
    fn get_remaining_points_works() {
        let metering = Arc::new(Metering::new(10, cost_function, MAX_MEMORY_COPY_SIZE));
        let mut compiler_config = Cranelift::default();
        compiler_config.push_middleware(metering);
        let mut store = Store::new(EngineBuilder::new(compiler_config));
        let module = Module::new(&store, bytecode()).unwrap();

        // Instantiate
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(10)
        );

        // First call
        //
        // Calling add_one costs 4 points. Here are the details of how it has been computed:
        // * `local.get $value` is a `Operator::LocalGet` which costs 1 point;
        // * `i32.const` is a `Operator::I32Const` which costs 1 point;
        // * `i32.add` is a `Operator::I32Add` which costs 2 points.
        let add_one: TypedFunction<i32, i32> = instance
            .exports
            .get_function("add_one")
            .unwrap()
            .typed(&store)
            .unwrap();
        add_one.call(&mut store, 1).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(6)
        );

        // Second call
        add_one.call(&mut store, 1).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(2)
        );

        // Third call fails due to limit
        assert!(add_one.call(&mut store, 1).is_err());
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Exhausted
        );
    }

    #[test]
    fn set_remaining_points_works() {
        let metering = Arc::new(Metering::new(10, cost_function, MAX_MEMORY_COPY_SIZE));
        let mut compiler_config = Cranelift::default();
        compiler_config.push_middleware(metering);
        let mut store = Store::new(EngineBuilder::new(compiler_config));
        let module = Module::new(&store, bytecode()).unwrap();

        // Instantiate
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(10)
        );
        let add_one: TypedFunction<i32, i32> = instance
            .exports
            .get_function("add_one")
            .unwrap()
            .typed(&store)
            .unwrap();

        // Increase a bit to have enough for 3 calls
        set_remaining_points(&mut store, &instance, 12);

        // Ensure we can use the new points now
        add_one.call(&mut store, 1).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(8)
        );

        add_one.call(&mut store, 1).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(4)
        );

        add_one.call(&mut store, 1).unwrap();
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(0)
        );

        assert!(add_one.call(&mut store, 1).is_err());
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Exhausted
        );

        // Add some points for another call
        set_remaining_points(&mut store, &instance, 4);
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(4)
        );
    }

    #[test]
    fn metering_works_for_loops() {
        const INITIAL_POINTS: u64 = 10_000;

        fn cost(operator: &Operator, _: Option<(u32, u32)>) -> u64 {
            match operator {
                Loop { .. } => 1000,
                Br { .. } | BrIf { .. } => 10,
                F64Const { .. } => 7,
                _ => 0,
            }
        }

        // Short loop

        let metering = Arc::new(Metering::new(INITIAL_POINTS, cost, MAX_MEMORY_COPY_SIZE));
        let mut compiler_config = Cranelift::default();
        compiler_config.push_middleware(metering);
        let mut store = Store::new(EngineBuilder::new(compiler_config));
        let module = Module::new(&store, bytecode()).unwrap();

        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        let short_loop: TypedFunction<(), ()> = instance
            .exports
            .get_function("short_loop")
            .unwrap()
            .typed(&store)
            .unwrap();
        short_loop.call(&mut store).unwrap();

        let points_used: u64 = match get_remaining_points(&mut store, &instance) {
            MeteringPoints::Exhausted => panic!("Unexpected exhausted"),
            MeteringPoints::Remaining(remaining) => INITIAL_POINTS - remaining,
        };

        assert_eq!(
            points_used,
            7 /* pre-loop instructions */ +
                1000 /* loop instruction */ + 50 /* five conditional breaks */
        );

        // Infinite loop

        let metering = Arc::new(Metering::new(INITIAL_POINTS, cost, MAX_MEMORY_COPY_SIZE));
        let mut compiler_config = Cranelift::default();
        compiler_config.push_middleware(metering);

        let mut store = Store::new(EngineBuilder::new(compiler_config));
        let module = Module::new(&store, bytecode()).unwrap();

        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        let infi_loop: TypedFunction<(), ()> = instance
            .exports
            .get_function("infi_loop")
            .unwrap()
            .typed(&store)
            .unwrap();
        infi_loop.call(&mut store).unwrap_err(); // exhausted leads to runtime error

        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Exhausted
        );
    }

    fn div_cost2(op: &Operator, _: Option<(u32, u32)>) -> u64 {
        match op {
            LocalGet { .. } | I32Const { .. } => 1,
            I32DivU { .. } => 2,
            _ => 0,
        }
    }

    #[test]
    fn metering_divu_regression() {
        // Disable length-clamp effects for this test.
        let metering = Arc::new(Metering::new(10, div_cost2, u32::MAX));

        let mut compiler_cfg = Cranelift::default();
        compiler_cfg.push_middleware(metering);
        let mut store = Store::new(EngineBuilder::new(compiler_cfg));

        // WASM: 10 / x
        let wasm = wat2wasm(
            br#"
            (module
              (type $bomb_t (func (param i32) (result i32)))
              (func $bomb (type $bomb_t) (param $x i32) (result i32)
                i32.const 10
                local.get $x
                i32.div_u)
              (export "bomb" (func $bomb)))
            "#,
        )
        .unwrap()
        .to_vec();

        let module = Module::new(&store, wasm).unwrap();
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();

        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(10)
        );

        let bomb: TypedFunction<i32, i32> = instance
            .exports
            .get_function("bomb")
            .unwrap()
            .typed(&store)
            .unwrap();

        // 10 / 2 = 5, costs 4 points → 6 left
        assert_eq!(bomb.call(&mut store, 2).unwrap(), 5);
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(6)
        );

        // 10 / 0 traps, still costs 4 points → 2 left
        assert!(bomb.call(&mut store, 0).is_err());
        assert_eq!(
            get_remaining_points(&mut store, &instance),
            MeteringPoints::Remaining(2)
        );
    }
}
