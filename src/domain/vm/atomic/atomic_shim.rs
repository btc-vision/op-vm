#![cfg(feature = "contract-threading")]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use futures::{future::select, pin_mut, FutureExt};
use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc,
    },
};
use tokio::task;
use wasmer::{AsStoreRef, FunctionEnvMut, Memory, RuntimeError, StoreRef};

use crate::domain::runner::{CustomEnv, InstanceWrapper, MAX_DIFF, NS_PER_HASH};

#[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
use crate::domain::vm::{prove_one_step, VdfState};

#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
use crate::domain::vm::{prove_one_step_zk_snark, VdfStateZkSnark, WaitQueue};

const OK: i32 = 0;
const TIMED_OUT: i32 = 1;
const NOT_EQUAL: i32 = 2;
const FAULT: i32 = -1;

trait AtomicInt: Send + Sync + 'static {
    type Int: Copy + Eq + Into<u64>;
    fn load_seq_acq(&self) -> Self::Int;
}

impl AtomicInt for AtomicU32 {
    type Int = u32;
    #[inline]
    fn load_seq_acq(&self) -> u32 {
        self.load(Ordering::Acquire)
    }
}

impl AtomicInt for AtomicU64 {
    type Int = u64;
    #[inline]
    fn load_seq_acq(&self) -> u64 {
        self.load(Ordering::Acquire)
    }
}

fn atomic_at<A: AtomicInt>(
    mem: &Memory,
    store: &impl AsStoreRef,
    addr: u64,
) -> Result<&'static A, RuntimeError> {
    let view = mem.view(store);
    let needed = addr
        .checked_add(size_of::<A>() as u64)
        .ok_or_else(|| RuntimeError::new("oob"))?;

    if needed > view.data_size() {
        return Err(RuntimeError::new("oob"));
    }

    // SAFETY: bounds verified above; `data_ptr` lives as long as `mem`
    Ok(unsafe { &*(view.data_ptr().add(addr as usize) as *const A) })
}

async fn deterministic_delay(seed: u64, diff: u64) {
    #[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
    {
        let mut vdf = VdfStateZkSnark::new(&seed.to_le_bytes());
        for n in 0..diff {
            prove_one_step_zk_snark(&mut vdf);
            if n & 0xFF == 0 {
                tokio::task::yield_now().await; // cooperative yield
            }
        }
    }

    #[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
    {
        let mut vdf = VdfState::new(&seed.to_le_bytes());
        for n in 0..diff {
            vdf.step();
            if n & 0xFF == 0 {
                tokio::task::yield_now().await; // cooperative yield
            }
        }
    }
}

fn notify_impl(
    ctx: &mut FunctionEnvMut<'_, CustomEnv>,
    addr: u64,
    count: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = ctx.data_and_store_mut();
    let inst = env
        .instance
        .as_ref()
        .cloned()
        .ok_or_else(|| RuntimeError::new("no instance"))?;

    let mem: &Memory =
        InstanceWrapper::get_memory(&inst.instance).map_err(|_| RuntimeError::new("no memory"))?;

    if addr >= mem.view(&store).data_size() {
        return Ok(0); // OOB => no-op
    }

    inst.futex_for(addr).notify(count)
}

async fn wait_async<A: AtomicInt>(
    mem: Memory,
    store: StoreRef<'_>,
    queue: Arc<WaitQueue>,
    addr: u64,
    expected: A::Int,
    timeout_ns: i64,
) -> Result<i32, RuntimeError>
where
    <A as AtomicInt>::Int: std::fmt::Display,
{
    println!("wait_async: addr = {addr}, expected = {expected}, timeout_ns = {timeout_ns}");
    if timeout_ns < 0 {
        return Err(RuntimeError::new("timeout must be >= 0"));
    }

    // unmatched value? – bail out quickly
    {
        let atomic = atomic_at::<A>(&mem, &store, addr)?;
        if atomic.load_seq_acq() != expected {
            return Ok(NOT_EQUAL);
        }
    }

    if timeout_ns == 0 {
        return Ok(TIMED_OUT);
    }

    /* build deterministic waiter + timer futures */
    let waiter = {
        let ticket = queue.ticket();
        queue.clone().wait_for_change(ticket).map(|_| ())
    };

    let diff = ((timeout_ns as u64) / NS_PER_HASH).clamp(1, MAX_DIFF);
    let seed = addr ^ expected.into() ^ diff;
    let timer = deterministic_delay(seed, diff);

    println!("Starting vdf timer: seed = {seed}, diff = {diff}");

    pin_mut!(waiter, timer);
    match select(waiter, timer).await {
        futures::future::Either::Left((_, _)) => Ok(OK),
        futures::future::Either::Right((_, _)) => Ok(TIMED_OUT),
    }
}

pub fn atomic_wait32(
    mut ctx: FunctionEnvMut<'_, CustomEnv>,
    ptr: u32,
    expected: i32,
    timeout_ns: i64,
) -> i32 {
    println!("atomic_wait32: ptr = {ptr}, expected = {expected}, timeout_ns = {timeout_ns}");
    do_wait::<AtomicU32>(&mut ctx, ptr as u64, expected as u32, timeout_ns)
}

pub fn atomic_wait64(
    mut ctx: FunctionEnvMut<'_, CustomEnv>,
    ptr: u32,
    expected: i64,
    timeout_ns: i64,
) -> i32 {
    println!("atomic_wait64: ptr = {ptr}, expected = {expected}, timeout_ns = {timeout_ns}");
    do_wait::<AtomicU64>(&mut ctx, ptr as u64, expected as u64, timeout_ns)
}

pub fn atomic_notify(mut ctx: FunctionEnvMut<'_, CustomEnv>, ptr: u32, count: i32) -> i32 {
    println!("atomic_notify: ptr = {ptr}, count = {count}");
    match notify_impl(&mut ctx, ptr as u64, count as u32) {
        Ok(woken) => woken as i32, // success
        Err(_runtime_err) => {
            println!("atomic_notify: error: {:?}", _runtime_err);
            FAULT
        }
    }
}

fn do_wait<A>(
    ctx: &mut FunctionEnvMut<'_, CustomEnv>,
    addr: u64,
    expected: A::Int,
    timeout_ns: i64,
) -> i32
where
    A: AtomicInt,
    <A as AtomicInt>::Int: std::fmt::Display,
{
    let (env, store_mut) = ctx.data_and_store_mut();
    let wrapper = match env.instance.as_ref().cloned() {
        Some(w) => w,
        None => return FAULT,
    };

    let mem = match InstanceWrapper::get_memory(&wrapper.instance) {
        Ok(m) => m.clone(),
        Err(_) => return FAULT,
    };

    let queue = wrapper.futex_for(addr);
    let store_ref = store_mut.as_store_ref();

    let res = task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(wait_async::<A>(
            mem, store_ref, queue, addr, expected, timeout_ns,
        ))
    });

    res.unwrap_or_else(|_| FAULT)
}
