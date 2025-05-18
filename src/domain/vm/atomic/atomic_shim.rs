#![cfg(feature = "contract-threading")]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

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

use crate::domain::runner::{CustomEnv, InstanceWrapper, ProvenState, MAX_DIFF, NS_PER_HASH};
use crate::domain::vm::{verify, Output, WaitQueue, OUTPUT_LEN};

#[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
use crate::domain::vm::{prove_one_step, VdfState};

#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
use crate::domain::vm::{prove_one_step_zk_snark as prove_one_step, VdfStateZkSnark as VdfState};

#[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
use crate::domain::vm::prove;
#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
use ark_bls12_381::Bls12_381;
#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
use ark_groth16::{Proof, VerifyingKey};
#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
use ark_serialize::{Compress, Validate};
#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
use {crate::domain::vm::prove, ark_serialize::CanonicalDeserialize};

const OK: i32 = 0;
const TIMED_OUT: i32 = 1;
const NOT_EQUAL: i32 = 2;
const NOT_AUTHORIZED: i32 = 3;
const FAULT: i32 = -1;

trait AtomicInt: Send + Sync + 'static {
    type Int: Copy + Eq + Into<u64>;
    fn load_seq_acq(&self) -> Self::Int;
}
impl AtomicInt for AtomicU32 {
    type Int = u32;
    fn load_seq_acq(&self) -> u32 {
        self.load(Ordering::Acquire)
    }
}
impl AtomicInt for AtomicU64 {
    type Int = u64;
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

    // SAFETY: bounds checked
    Ok(unsafe { &*(view.data_ptr().add(addr as usize) as *const A) })
}

fn read_slice<'a>(
    mem: &'a Memory,
    store: &impl AsStoreRef,
    ptr: u32,
    len: u32,
) -> Result<&'a [u8], RuntimeError> {
    let view = mem.view(store);
    let offset = ptr as usize;
    let end = offset
        .checked_add(len as usize)
        .ok_or_else(|| RuntimeError::new("oob"))?;

    if end > view.data_size() as usize {
        return Err(RuntimeError::new("oob"));
    }

    // SAFETY: bounds checked
    Ok(unsafe { std::slice::from_raw_parts(view.data_ptr().add(offset), len as usize) })
}

fn little_endian_u64(x: u64) -> [u8; 8] {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&x.to_le_bytes());
    buf
}

async fn deterministic_delay(seed: u64, diff: u64) {
    let mut vdf = VdfState::new(&little_endian_u64(seed));
    for n in 0..diff {
        prove_one_step(&mut vdf);

        if n & 0xFF == 0 {
            tokio::task::yield_now().await;
        }
    }
}

#[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
fn parse_proof(buf: &[u8]) -> Option<([u8; OUTPUT_LEN], BigUint)> {
    if buf.len() < OUTPUT_LEN {
        return None;
    }

    let mut y = [0u8; OUTPUT_LEN];
    y.copy_from_slice(&buf[..OUTPUT_LEN]);

    let pi = BigUint::from_bytes_be(&buf[OUTPUT_LEN..]);
    Some((y, pi))
}

#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
fn parse_snark(buf: &[u8]) -> Option<(Output, Proof<Bls12_381>)> {
    if buf.len() < OUTPUT_LEN {
        return None;
    }

    let mut y = [0u8; OUTPUT_LEN];
    y.copy_from_slice(&buf[..OUTPUT_LEN]);

    let proof = Proof::<Bls12_381>::deserialize_compressed(&buf[OUTPUT_LEN..]).ok()?;
    Some((y, proof))
}

#[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
fn deserialize_vk(bytes: &[u8]) -> Option<VerifyingKey<Bls12_381>> {
    use std::io::Cursor;
    CanonicalDeserialize::deserialize_with_mode(
        &mut Cursor::new(bytes),
        Compress::Yes,
        Validate::Yes,
    )
    .ok()
}

async fn wait_async<A: AtomicInt>(
    mem: Memory,
    store: StoreRef<'_>,
    queue: Arc<WaitQueue>,
    addr: u64,
    expected: A::Int,
    timeout_ns: i64,
    proof_bytes: Vec<u8>,
    vk_bytes: Vec<u8>,
) -> Result<i32, RuntimeError>
where
    <A as AtomicInt>::Int: std::fmt::Display,
{
    /* quick mismatch check */
    if timeout_ns < 0 {
        return Err(RuntimeError::new("timeout must be >= 0"));
    }

    {
        let atomic = atomic_at::<A>(&mem, &store, addr)?;
        if atomic.load_seq_acq() != expected {
            return Ok(NOT_EQUAL);
        }
    }

    /* zero timeout → immediate failure (spec) */
    if timeout_ns == 0 {
        return Ok(TIMED_OUT);
    }

    if !proof_bytes.is_empty() {
        let diff = ((timeout_ns as u64) / NS_PER_HASH).clamp(1, MAX_DIFF);
        let seed = addr ^ expected.into() ^ diff;

        #[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
        match parse_proof(&proof_bytes) {
            None => return Ok(NOT_AUTHORIZED), // bad format
            Some((y, pi)) if verify(&little_endian_u64(seed), diff, &y, &pi) => {
                /* proof is VALID → keep going and *still* run the delay */
            }
            _ => return Ok(NOT_AUTHORIZED), // wrong proof
        }

        #[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
        match parse_snark(&proof_bytes) {
            None => return Ok(NOT_AUTHORIZED),
            Some((y, proof)) => {
                let vk = if vk_bytes.is_empty() {
                    return Ok(NOT_AUTHORIZED);
                } else {
                    match deserialize_vk(&vk_bytes) {
                        Some(v) => Box::leak(Box::new(v)), // 'static lifetime hack
                        None => return Ok(NOT_AUTHORIZED),
                    }
                };

                if !verify(&little_endian_u64(seed), &y, &proof, &vk) {
                    return Ok(NOT_AUTHORIZED);
                }
            }
        }
    }

    let waiter = {
        let ticket = queue.ticket();
        queue.clone().wait_for_change(ticket).map(|_| ())
    };

    let diff = ((timeout_ns as u64) / NS_PER_HASH).clamp(1, MAX_DIFF);
    let seed = addr ^ expected.into() ^ diff;
    let timer = deterministic_delay(seed, diff);

    pin_mut!(waiter, timer);
    match select(waiter, timer).await {
        futures::future::Either::Left((_, _)) => Ok(OK),
        futures::future::Either::Right((_, _)) => Ok(TIMED_OUT),
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
        return Ok(0);
    }

    inst.futex_for(addr).notify(count)
}

macro_rules! impl_wait_fn {
    ($name:ident, $atomic:ty, $val_ty:ty) => {
        #[allow(non_snake_case)]
        pub fn $name(
            mut ctx: FunctionEnvMut<'_, CustomEnv>,
            ptr: u32,
            expected: $val_ty,
            timeout_ns: i64,
            proof_ptr: u32,
            proof_len: u32,
            vk_ptr: u32,
            vk_len: u32,
        ) -> i32 {
            use std::io::Cursor;

            let (env, store_mut) = ctx.data_and_store_mut();
            let wrapper = match env.instance.as_ref().cloned() {
                Some(w) => w,
                None => return FAULT,
            };
            let mem = match InstanceWrapper::get_memory(&wrapper.instance) {
                Ok(m) => m.clone(),
                Err(_) => return FAULT,
            };

            let raw_proof = match read_slice(&mem, &store_mut, proof_ptr, proof_len) {
                Ok(s) => s.to_vec(),
                Err(_) => return FAULT,
            };

            let raw_vk = match read_slice(&mem, &store_mut, vk_ptr, vk_len) {
                Ok(s) => s.to_vec(),
                Err(_) => return FAULT,
            };

            let want_proof =
                env.return_proofs && !raw_proof.is_empty() && raw_proof.iter().all(|b| *b == 0);

            // send empty Vec to wait_async so it behaves like “no proof”
            let proof_vec_for_wait = if want_proof {
                Vec::new()
            } else {
                raw_proof.clone()
            };

            let vk_vec_for_wait = if want_proof {
                Vec::new()
            } else {
                raw_vk.clone()
            };

            let queue = wrapper.futex_for(ptr as u64);
            let store_ref = store_mut.as_store_ref();
            let res = task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(wait_async::<$atomic>(
                    mem,
                    store_ref,
                    queue,
                    ptr as u64,
                    expected as _,
                    timeout_ns,
                    proof_vec_for_wait,
                    vk_vec_for_wait,
                ))
            });

            let code = res.unwrap_or_else(|_| FAULT);

            if want_proof && code != FAULT {
                let diff = ((timeout_ns as u64) / NS_PER_HASH).clamp(1, MAX_DIFF);
                let seed = (ptr as u64) ^ (expected as u64) ^ diff;

                #[cfg(all(feature = "vdf", not(feature = "vdf-zk-snark")))]
                {
                    let (y, pi) = prove(&little_endian_u64(seed), diff);
                    let mut bytes = Vec::with_capacity(OUTPUT_LEN + pi.to_bytes_be().len());
                    bytes.extend_from_slice(&y);
                    bytes.extend_from_slice(&pi.to_bytes_be());
                    env.proofs.push(bytes);
                }

                #[cfg(all(feature = "vdf-zk-snark", not(feature = "vdf")))]
                {
                    use ark_serialize::{CanonicalSerialize, Compress};

                    let (y, proof, vk) = match prove(&little_endian_u64(seed), diff) {
                        Ok(pair) => pair,
                        Err(_) => return FAULT,
                    };

                    let mut buf = Vec::with_capacity(proof.serialized_size(Compress::Yes));
                    let mut vk_compressed = Vec::with_capacity(vk.serialized_size(Compress::Yes));
                    if vk
                        .serialize_compressed(&mut Cursor::new(&mut vk_compressed))
                        .is_err()
                    {
                        println!("Failed to serialize vk");
                        return FAULT;
                    }

                    if proof
                        .serialize_compressed(&mut Cursor::new(&mut buf))
                        .is_err()
                    {
                        println!("Failed to serialize proof");
                        return FAULT;
                    }

                    let output_proof = [&y[..], &buf[..]].concat();
                    env.proofs.push(ProvenState {
                        proof: output_proof,
                        vk: vk_compressed,
                    });
                }
            }

            if env.return_proofs && !raw_proof.is_empty() && !want_proof {
                env.proofs.push(ProvenState {
                    proof: raw_proof,
                    vk: raw_vk,
                });
            }

            code
        }
    };
}

impl_wait_fn!(atomic_wait32, AtomicU32, i32);
impl_wait_fn!(atomic_wait64, AtomicU64, i64);

/* the notify ABI never changed */
pub fn atomic_notify(mut ctx: FunctionEnvMut<'_, CustomEnv>, ptr: u32, count: i32) -> i32 {
    match notify_impl(&mut ctx, ptr as u64, count as u32) {
        Ok(w) => w as i32,
        Err(_) => FAULT,
    }
}
