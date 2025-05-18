#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]

use ark_std::rand;
use ark_std::rand::RngCore as Rnd;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng, TryRngCore};

pub struct FixedChaCha(ChaCha20Rng);

impl SeedableRng for FixedChaCha {
    type Seed = <ChaCha20Rng as SeedableRng>::Seed;
    fn from_seed(seed: Self::Seed) -> Self {
        Self(ChaCha20Rng::from_seed(seed))
    }
    fn seed_from_u64(state: u64) -> Self {
        Self(ChaCha20Rng::seed_from_u64(state))
    }
}

impl Rnd for FixedChaCha {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest).map_err(|e| rand::Error::new(e))
    }
}
