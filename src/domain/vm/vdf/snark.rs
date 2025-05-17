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

pub type Output = [u8; 32];

#[inline]
pub fn sha256_array(data: &[u8]) -> Output {
    let mut out = [0u8; 32];
    out.copy_from_slice(Sha256::digest(data).as_slice());

    // flip the 4 bytes inside each 32-bit word
    for chunk in out.chunks_mut(4) {
        chunk.reverse();
    }
    out
}

#[derive(Clone)]
struct Sha256IterCircuit {
    init: Vec<u8>,
    expected: Vec<u8>,
    iterations: usize,
}

#[cfg(test)]
fn dump(tag: &str, bytes: &[UInt8<Fr>]) {
    let v: Vec<u8> = bytes.iter().map(|b| b.value().unwrap()).collect();
    println!("{tag}: {}", hex::encode(v));
}

impl ConstraintSynthesizer<Fr> for Sha256IterCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let mut state = UInt8::new_input_vec(cs.clone(), &self.init)?;
        let expected_vec = UInt8::new_input_vec(cs, &self.expected)?;

        for i in 0..self.iterations {
            let digest = Sha256Gadget::<Fr>::evaluate(&UnitVar::default(), &state)?;
            state = digest.to_bytes_le()?;
            #[cfg(test)]
            dump(&format!("gadget iter {i}"), &state);
        }

        #[cfg(test)]
        dump("expected", &expected_vec);

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
    if t == 0 {
        return sha256_array(seed);
    }
    let mut h = sha256_array(seed);
    for i in 1..t {
        println!("native digest at iter {i}: {}", hex::encode(h));
        h = sha256_array(&h);
    }
    h
}

pub fn prove(seed: &[u8], t: u64) -> Result<(Output, Proof<Bls12_381>), SynthesisError> {
    if t == 0 {
        return Ok((sha256_array(seed), Proof::<Bls12_381>::default()));
    }

    let out = expected_output(seed, t);
    let cir = Sha256IterCircuit {
        init: seed.to_vec(),
        expected: out.to_vec(),
        iterations: t as usize,
    };

    let (pk, _) = gen_params(t as usize).expect("parameter generation failed");
    let mut rng = StdRng::seed_from_u64(99);
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(cir, &pk, &mut rng)
        .map_err(|e| {
            println!("Proof generation error: {:?}", e);
            e
        })?;
    Ok((out, proof))
}

pub fn verify(seed: &[u8], t: u64, out: &Output, proof: &Proof<Bls12_381>) -> bool {
    if t == 0 {
        return out == &sha256_array(seed);
    }

    let (_, vk) = match gen_params(t as usize) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let mut public: Vec<Fr> = seed.iter().map(|&b| Fr::from(b as u64)).collect();
    public.extend(out.iter().map(|&b| Fr::from(b as u64)));

    let pvk = prepare_verifying_key(&vk);
    Groth16::<Bls12_381>::verify_proof(&pvk, proof, &public)
        .map_err(|e| {
            println!("Verification error: {:?}", e);

            false
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_small() {
        for t in 0u64..=4 {
            let seed = b"btc-compatible-seed-phrase-32-bytes!";
            let (y, pi) = prove(seed, t).expect("prove failed");
            assert!(verify(seed, t, &y, &pi));
        }
    }
}
