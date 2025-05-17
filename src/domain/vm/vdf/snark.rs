#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]

use anyhow::Result;
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::sha256::constraints::{Sha256Gadget, UnitVar};
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_ff::{PrimeField, Zero};
use ark_groth16::{prepare_verifying_key, Groth16, Proof, ProvingKey, VerifyingKey};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_std::vec::Vec;
use bitcoin::hex::DisplayHex;
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
    iterations: u64,
    pub_hash_fr: Fr,
}

impl ConstraintSynthesizer<Fr> for Sha256IterCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let mut state = UInt8::new_witness_vec(cs.clone(), &self.init)?;

        let parms = &UnitVar::default();
        for _ in 0..self.iterations {
            let digest = Sha256Gadget::<Fr>::evaluate(parms, &state)?;
            state = digest.0;
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
fn hash_to_fr(digest: &[u8]) -> Fr {
    Fr::from_le_bytes_mod_order(digest)
}

fn gen_params(t: u64, seed_len: usize) -> Result<(ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>)> {
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
    let mut cur = seed.to_vec();
    let mut digest = [0u8; 32];

    println!("looping {} times", t);

    for _ in 0..t {
        digest = sha256_array(&cur);
        cur.clear();
        cur.extend_from_slice(&digest);
        //cur.reverse();

        println!("v -> {:?}", cur);
    }

    digest
}

pub fn prove(seed: &[u8], t: u64) -> Result<(Output, Proof<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let y = expected_output(seed, t);

    let mut buf = seed.to_vec();
    buf.extend_from_slice(&y);
    let pub_digest = sha256_array(&buf);
    let h_fr = hash_to_fr(&pub_digest);

    let circ = Sha256IterCircuit {
        init: seed.to_vec(),
        iterations: t,
        pub_hash_fr: h_fr,
    };

    let (pk, vk) = gen_params(t, seed.len())?;

    let mut rng = StdRng::seed_from_u64(99);
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(circ, &pk, &mut rng)
        .map_err(|e| anyhow::anyhow!("Failed to create proof: {:?}", e))?;

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
    let pub_digest = sha256_array(&buf);
    let h_fr = hash_to_fr(&pub_digest);

    let pvk = prepare_verifying_key(vk);
    let prepared_inputs =
        Groth16::<Bls12_381>::prepare_inputs(&pvk, &[h_fr]).expect("prepare inputs failed");

    Groth16::<Bls12_381>::verify_proof_with_prepared_inputs(&pvk, proof, &prepared_inputs)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vm::OUTPUT_LEN;
    use ark_serialize::{CanonicalSerialize, Compress};
    use std::io::Cursor;

    #[test]
    fn roundtrip_small() {
        let t = 1;
        let seed = b"btc-compatible-seed-phrase-32-bytes!";

        let (y, proof, vk) = prove(seed, t).expect("prove failed");
        assert!(verify(seed, &y, &proof, &vk));

        let mut buf = Vec::with_capacity(OUTPUT_LEN + proof.serialized_size(Compress::Yes));

        let mut vk_compressed = Vec::with_capacity(vk.serialized_size(Compress::Yes));
        if vk
            .serialize_compressed(&mut Cursor::new(&mut vk_compressed))
            .is_err()
        {
            assert!(false);
        }

        buf.extend_from_slice(&y);

        if proof
            .serialize_compressed(&mut Cursor::new(&mut buf))
            .is_err()
        {
            assert!(false);
        }

        println!("Serialized proof: {}", buf.to_lower_hex_string());
        println!("Serialized vk: {}", vk_compressed.to_lower_hex_string());
    }
}
