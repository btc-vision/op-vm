#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use crate::domain::vm::{poseidon_array, Output};
use wasmer::RuntimeError;

#[derive(Clone)]
pub struct VdfStateZkSnark {
    acc: Output,
    steps: u64,
}

impl VdfStateZkSnark {
    /// State starts at `sha256_array(seed)` ⇒ one hash already done.
    #[inline]
    pub fn new(seed: &[u8]) -> Result<Self, RuntimeError> {
        let array = poseidon_array(seed).map_err(|e| RuntimeError::new(e.to_string()))?;

        Ok(Self {
            acc: array,
            steps: 1,
        })
    }

    /// One extra SHA-256 round (little-endian output fed back in)
    #[inline]
    pub fn step(&mut self) -> Result<(), RuntimeError> {
        self.acc = poseidon_array(&self.acc).map_err(|e| RuntimeError::new(e.to_string()))?;
        self.steps += 1;

        Ok(())
    }

    #[inline]
    pub fn output(&self) -> Output {
        self.acc
    }
}

/// Convenience wrapper for the WASM VM
#[inline]
pub fn prove_one_step_zk_snark(st: &mut VdfStateZkSnark) -> Result<(), RuntimeError> {
    Ok(st.step()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vm::{expected_output, fr_from_bytes, fr_to_bytes};

    #[test]
    fn incremental_equals_bulk() {
        let seed = b"inc-test";
        let t: u64 = 20;

        let mut s = VdfStateZkSnark::new(seed).expect("creating state");
        for _ in 1..t {
            s.step().expect("test failed");
        }

        let seed_fr = fr_from_bytes(seed);

        let output = expected_output(&seed_fr, t).expect("expected output");
        let bytes = fr_to_bytes(&output);

        assert_eq!(s.output(), bytes);
    }
}
