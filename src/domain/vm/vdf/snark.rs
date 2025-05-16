#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]

use anyhow::Result;
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::sha256::constraints::{Sha256Gadget, UnitVar};
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_groth16::{prepare_verifying_key, Groth16, Proof, ProvingKey, VerifyingKey};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_std::vec::Vec;
use sha2::{Digest, Sha256};

/// 32-byte digest returned by the VDF
pub type Output = [u8; 32];

/// Circuit proving `expected = H^iterations(init)`.
#[derive(Clone)]
struct Sha256IterCircuit {
    init: Vec<u8>,
    expected: Vec<u8>,
    iterations: usize,
}

impl ConstraintSynthesizer<Fr> for Sha256IterCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let mut state = UInt8::new_input_vec(cs.clone(), &self.init)?;
        let expected_vec = UInt8::new_input_vec(cs, &self.expected)?;

        for _ in 0..self.iterations {
            let digest = Sha256Gadget::<Fr>::evaluate(&UnitVar::default(), &state)?;
            state = digest.to_bytes_le()?;
        }
        for (a, b) in state.iter().zip(expected_vec.iter()) {
            a.enforce_equal(b)?;
        }
        Ok(())
    }
}

fn gen_params(iters: usize) -> Result<(ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let dummy = Sha256IterCircuit {
        init: vec![0; 32],
        expected: vec![0; 32],
        iterations: iters,
    };
    let mut rng = StdRng::seed_from_u64(42);
    let pk = Groth16::<Bls12_381>::generate_random_parameters_with_reduction(dummy, &mut rng)?;
    Ok((pk.clone(), pk.vk))
}

pub fn expected_output(seed: &[u8], t: u64) -> Output {
    let mut h = Sha256::digest(seed);
    for _ in 1..=t {
        h = Sha256::digest(&h);
    }
    h.into()
}

pub fn prove(seed: &[u8], t: u64) -> (Output, Proof<Bls12_381>) {
    let out = expected_output(seed, t);
    let cir = Sha256IterCircuit {
        init: seed.to_vec(),
        expected: out.to_vec(),
        iterations: t as usize,
    };

    let (pk, _) = gen_params(t as usize).expect("param‐gen failed");
    let mut rng = StdRng::seed_from_u64(99);
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(cir, &pk, &mut rng)
        .expect("proving failed");
    (out, proof)
}

pub fn verify(seed: &[u8], t: u64, out: &Output, proof: &Proof<Bls12_381>) -> bool {
    let (_, vk) = match gen_params(t as usize) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let mut public = bits(seed);
    public.extend(bits(out));

    let pvk = prepare_verifying_key(&vk);
    Groth16::<Bls12_381>::verify_proof(&pvk, proof, &public).unwrap_or(false)
}

fn bits(bytes: &[u8]) -> Vec<Fr> {
    let mut v = Vec::with_capacity(bytes.len() * 8);
    for &b in bytes {
        for i in 0..8 {
            v.push(Fr::from(((b >> i) & 1) as u64));
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_small() {
        for t in 0u64..=4 {
            let seed = b"btc-compatible-seed-phrase-32-bytes!";
            let (y, pi) = prove(seed, t);
            assert!(verify(seed, t, &y, &pi));
        }
    }
}
