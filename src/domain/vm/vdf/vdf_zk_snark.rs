#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use crate::domain::vm::{sha256_array, Output};

pub const OUTPUT_LEN: usize = 32;

#[derive(Clone)]
pub struct VdfStateZkSnark {
    acc: Output,
    steps: u64,
}

impl VdfStateZkSnark {
    /// State starts at `sha256_array(seed)` ⇒ one hash already done.
    #[inline]
    pub fn new(seed: &[u8]) -> Self {
        Self {
            acc: sha256_array(seed),
            steps: 1,
        }
    }

    /// One extra SHA-256 round (little-endian output fed back in)
    #[inline]
    pub fn step(&mut self) {
        self.acc = sha256_array(&self.acc);
        self.steps += 1;
    }

    #[inline]
    pub fn output(&self) -> Output {
        self.acc
    }

    /*pub fn generate_proof(
        &self,
        seed: &[u8],
        t: u64,
    ) -> Result<(Output, Proof<Bls12_381>), RuntimeError> {
        if self.steps == 0 {
            return Ok((self.acc, Proof::default()));
        }
        prove(seed, t).map_err(|_| RuntimeError::new("Failed to generate proof"))
    }

    /// Verify a proof against the current accumulator
    pub fn verify(&self, seed: &[u8], proof: Proof<Bls12_381>) -> bool {
        if self.steps == 0 {
            return false;
        }

        let t = self.steps - 1;
        verify(seed, t, &self.acc, &proof)
    }*/
}

/// Convenience wrapper for the WASM VM
#[inline]
pub fn prove_one_step_zk_snark(st: &mut VdfStateZkSnark) {
    st.step();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vm::expected_output;

    #[test]
    fn incremental_equals_bulk() {
        let seed = b"inc-test";
        let t = 20;

        let mut s = VdfStateZkSnark::new(seed);
        for _ in 1..t {
            s.step();
        }

        assert_eq!(s.output(), expected_output(seed, t));
    }
}
