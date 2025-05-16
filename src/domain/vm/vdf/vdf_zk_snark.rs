#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)] // important
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use crate::domain::vm::{prove, verify, Output};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct VdfStateZkSnark {
    acc: Output,
    steps: u64,
}

impl VdfStateZkSnark {
    #[inline]
    pub fn new(seed: &[u8]) -> Self {
        Self {
            acc: Sha256::digest(seed).into(),
            steps: 1,
        }
    }

    #[inline]
    pub fn step(&mut self) {
        self.acc = Sha256::digest(&self.acc).into();
        self.steps += 1;
    }

    #[inline]
    pub fn output(&self) -> Output {
        self.acc
    }

    pub fn verify(&self, seed: &[u8]) -> bool {
        if self.steps == 0 {
            return false;
        }

        let t = self.steps - 1; // because new() already did 1 hash
        let proof_bytes = prove(seed, t).1; // produce proof off-chain or pass in
        verify(seed, t, &self.acc, &proof_bytes)
    }
}

#[inline]
pub fn prove_one_step_zk_snark(st: &mut VdfStateZkSnark) {
    st.step();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vm::expected_output;
    #[test]
    fn roundtrip_small() {
        for t in 0u64..=4 {
            let seed = b"btc-compatible-seed-phrase-32-bytes!";
            let (y, pi) = prove(seed, t);
            assert!(verify(seed, t, &y, &pi));
        }
    }

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
