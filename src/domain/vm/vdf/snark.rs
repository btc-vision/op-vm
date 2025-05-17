#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]

use anyhow::Result;
use ark_bls12_381::Bls12_381;
use ark_bls12_381::Fr;
use ark_crypto_primitives::crh::sha256::constraints::{Sha256Gadget, UnitVar};
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_groth16::{prepare_verifying_key, Groth16, Proof, ProvingKey, VerifyingKey};
use ark_r1cs_std::boolean::Boolean;
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
    out
}

#[derive(Clone)]
struct Sha256IterCircuit {
    init: Vec<u8>,
    iterations: usize,
    pub_hash_fr: Fr,
}

impl ConstraintSynthesizer<Fr> for Sha256IterCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let mut state = UInt8::new_witness_vec(cs.clone(), &self.init)?;

        for _ in 0..self.iterations {
            let digest = Sha256Gadget::<Fr>::evaluate(&UnitVar::default(), &state)?;
            state = digest.to_bytes_le()?;
        }

        let mut input = UInt8::new_witness_vec(cs.clone(), &self.init)?;
        input.extend_from_slice(&state);

        let pub_digest = Sha256Gadget::<Fr>::evaluate(&UnitVar::default(), &input)?;
        let digest_bytes = pub_digest.to_bytes_le()?;

        let bits: Vec<Boolean<Fr>> = digest_bytes
            .iter()
            .flat_map(|b| b.to_bits_le().unwrap())
            .collect();

        let fr_in_circuit = Boolean::le_bits_to_fp(&bits)?;
        let fr_public =
            ark_r1cs_std::fields::fp::FpVar::<Fr>::new_input(cs, || Ok(self.pub_hash_fr))?;
        fr_in_circuit.enforce_equal(&fr_public)?;

        Ok(())
    }
}

#[inline]
fn hash_to_fr(digest_be: &[u8]) -> Fr {
    let mut le = digest_be.to_vec();
    le.reverse();
    Fr::from_le_bytes_mod_order(&le)
}

fn gen_params(
    t: usize,
    seed_len: usize,
) -> Result<(ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let dummy = Sha256IterCircuit {
        init: vec![0u8; seed_len],
        iterations: t,
        pub_hash_fr: Fr::zero(),
    };

    let mut rng = StdRng::seed_from_u64(42);
    let pk = Groth16::<Bls12_381>::generate_random_parameters_with_reduction(dummy, &mut rng)?;
    Ok((pk.clone(), pk.vk))
}

pub fn expected_output(seed: &[u8], t: u64) -> Output {
    /*if t == 0 {
        return sha256_array(seed);
    }*/

    let mut h = sha256_array(seed);
    for i in 1..t {
        println!("native digest at iter {i}: {}", hex::encode(h));
        h = sha256_array(&h);
    }
    h
}

pub fn prove(seed: &[u8], t: u64) -> Result<(Output, Proof<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let y = expected_output(seed, t);
    let mut buf = seed.to_vec();
    buf.extend_from_slice(&y);
    let h_fr = hash_to_fr(&buf);

    let circ = Sha256IterCircuit {
        init: seed.to_vec(),
        iterations: t as usize,
        pub_hash_fr: h_fr,
    };

    let (pk, vk) = gen_params(t as usize, seed.len()).map_err(|e| {
        println!("Parameter generation error: {:?}", e);
        e
    })?;

    let mut rng = StdRng::seed_from_u64(99);
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(circ, &pk, &mut rng)
        .map_err(|e| {
            println!("Proof generation error: {:?}", e);
            e
        })?;

    Ok((y, proof, vk))
}

pub fn verify(
    seed: &[u8],
    y: &Output,
    proof: &Proof<Bls12_381>,
    vk: &VerifyingKey<Bls12_381>,
) -> bool {
    let mut buf = seed.to_vec();
    buf.extend_from_slice(y);

    let h_fr = hash_to_fr(&buf);
    let pvk = prepare_verifying_key(vk);
    let prepared_inputs = Groth16::<Bls12_381>::prepare_inputs(&pvk, &[h_fr])
        .map_err(|e| {
            println!("Error preparing inputs: {:?}", e);
            e
        })
        .expect("Prepare input error");

    Groth16::<Bls12_381>::verify_proof_with_prepared_inputs(&pvk, proof, &prepared_inputs)
        .map_err(|e| {
            println!("Proof verification error: {:?}", e);
            e
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_small() {
        let t = 4;
        let seed = b"btc-compatible-seed-phrase-32-bytes!";

        let (y, pi, vk) = prove(seed, t).expect("prove failed");
        assert!(verify(seed, &y, &pi, &vk));
    }
}
