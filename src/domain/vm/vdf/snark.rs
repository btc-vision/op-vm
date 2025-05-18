#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]

use crate::domain::vm::vdf::fixed_cha_cha::FixedChaCha;
use anyhow::{Context, Result};
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_crypto_primitives::{
    crh::{
        poseidon::{
            constraints::{CRHGadget as PoseidonGadget, CRHParametersVar as PoseidonParamsVar},
            CRH as PoseidonCRH,
        },
        CRHScheme,
    },
    sponge::poseidon::PoseidonConfig,
};
use ark_ff::{BigInteger, PrimeField, UniformRand, Zero};
use ark_groth16::{prepare_verifying_key, Groth16, Proof, ProvingKey, VerifyingKey};
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::vec::Vec;
use once_cell::sync::Lazy;
use rand_core::SeedableRng;
use sha2::{Digest, Sha256};
use wasmer::RuntimeError;

pub const OUTPUT_LEN: usize = 32;
pub type Output = [u8; 32];

fn seeded_rng() -> FixedChaCha {
    const CTX: &[u8] = b"OPNET_POSEIDON_BLS12_381_WIDTH3_RF8_RP57_V1";
    let hash = Sha256::digest(CTX);
    let mut lo = [0u8; 8];
    lo.copy_from_slice(&hash[..8]);
    FixedChaCha::seed_from_u64(u64::from_le_bytes(lo))
}

fn build_poseidon_params() -> PoseidonConfig<Fr> {
    const W: usize = 3;
    const R_F: usize = 8;
    const R_P: usize = 57;
    const ALPHA: u64 = 5;

    let mut rng = seeded_rng();
    let ark: Vec<Vec<Fr>> = (0..R_F + R_P)
        .map(|_| (0..W).map(|_| Fr::rand(&mut rng)).collect())
        .collect();
    let mds: Vec<Vec<Fr>> = (0..W)
        .map(|_| (0..W).map(|_| Fr::rand(&mut rng)).collect())
        .collect();

    PoseidonConfig {
        full_rounds: R_F,
        partial_rounds: R_P,
        alpha: ALPHA,
        ark,
        mds,
        rate: 2,
        capacity: 1,
    }
}

static POSEIDON_PARAMS: Lazy<PoseidonConfig<Fr>> = Lazy::new(build_poseidon_params);

#[inline]
pub fn fr_from_bytes(b: &[u8]) -> Fr {
    Fr::from_le_bytes_mod_order(b)
}

#[inline]
pub fn fr_to_bytes(fr: &Fr) -> Output {
    let mut out = [0u8; 32];
    let limbs = fr.into_bigint().to_bytes_le();
    out[..limbs.len()].copy_from_slice(&limbs);
    out
}

#[inline]
fn poseidon_hash_native(inputs: &[Fr]) -> Result<Fr, RuntimeError> {
    PoseidonCRH::<Fr>::evaluate(&POSEIDON_PARAMS, inputs)
        .map_err(|e| RuntimeError::new(e.to_string()))
}

#[derive(Clone)]
struct PoseidonIterCircuit {
    init: Fr,
    iterations: u64,
    pub_hash_fr: Fr,
}

impl ConstraintSynthesizer<Fr> for PoseidonIterCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let params_var = PoseidonParamsVar::new_constant(cs.clone(), &*POSEIDON_PARAMS)?;

        let mut state = FpVar::<Fr>::new_witness(cs.clone(), || Ok(self.init))?;
        for _ in 0..self.iterations {
            state = PoseidonGadget::<Fr>::evaluate(&params_var, &[state.clone()])?;
        }

        let seed_var = FpVar::<Fr>::new_witness(cs.clone(), || Ok(self.init))?;
        let pub_var = PoseidonGadget::<Fr>::evaluate(&params_var, &[seed_var, state])?;
        let pub_in = FpVar::<Fr>::new_input(cs, || Ok(self.pub_hash_fr))?;
        pub_var.enforce_equal(&pub_in)?;
        Ok(())
    }
}

fn gen_keys(t: u64) -> Result<(ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let dummy = PoseidonIterCircuit {
        init: Fr::zero(),
        iterations: t,
        pub_hash_fr: Fr::zero(),
    };

    let mut rng = FixedChaCha::seed_from_u64(42);
    let pk = Groth16::<Bls12_381>::generate_random_parameters_with_reduction(dummy, &mut rng)
        .context("Groth16 parameter generation failed")?;
    Ok((pk.clone(), pk.vk))
}

pub fn expected_output(seed: &Fr, t: u64) -> Result<Fr> {
    let mut cur = *seed;
    for _ in 0..t {
        cur = poseidon_hash_native(&[cur])?;
    }
    Ok(cur)
}

pub fn prove(
    seed_bytes: &[u8],
    t: u64,
) -> Result<(Output, Proof<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let seed_fr = fr_from_bytes(seed_bytes);
    let y_fr = expected_output(&seed_fr, t)?;
    let pub_hash_fr = poseidon_hash_native(&[seed_fr, y_fr]).context("computing public hash")?;

    let circ = PoseidonIterCircuit {
        init: seed_fr,
        iterations: t,
        pub_hash_fr,
    };

    let (pk, vk) = gen_keys(t)?;
    let mut rng = FixedChaCha::seed_from_u64(99);
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(circ, &pk, &mut rng)
        .context("Groth16 proving failed")?;

    Ok((fr_to_bytes(&y_fr), proof, vk))
}

pub fn verify(
    seed_bytes: &[u8],
    y_bytes: &Output,
    proof: &Proof<Bls12_381>,
    vk: &VerifyingKey<Bls12_381>,
) -> Result<bool> {
    let seed_fr = fr_from_bytes(seed_bytes);
    let y_fr = fr_from_bytes(y_bytes);
    let pub_fr = poseidon_hash_native(&[seed_fr, y_fr]).context("computing public hash")?;

    let pvk = prepare_verifying_key(vk);
    let prepared_inputs =
        Groth16::<Bls12_381>::prepare_inputs(&pvk, &[pub_fr]).context("prepare_inputs failed")?;

    let ok = Groth16::<Bls12_381>::verify_proof_with_prepared_inputs(&pvk, proof, &prepared_inputs)
        .context("Groth16 verification error")?;

    Ok(ok)
}

pub fn poseidon_array(seed: &[u8]) -> Result<Output> {
    let seed_fr = fr_from_bytes(seed);
    let y_fr = poseidon_hash_native(&[seed_fr])?;
    Ok(fr_to_bytes(&y_fr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_serialize::{CanonicalSerialize, Compress};
    use bitcoin::hex::{Case, DisplayHex};
    use std::io::Cursor;

    #[test]
    fn roundtrip_small() {
        let t = 5;
        let seed = b"btc-compatible-seed-phrase-32-bytes!";

        let (y, proof, vk) = prove(seed, t).expect("prove failed");
        assert!(verify(seed, &y, &proof, &vk).unwrap());

        let mut buf = Vec::with_capacity(proof.serialized_size(Compress::Yes));
        let mut vk_compressed = Vec::with_capacity(vk.serialized_size(Compress::Yes));

        vk.serialize_compressed(&mut Cursor::new(&mut vk_compressed))
            .unwrap();
        proof
            .serialize_compressed(&mut Cursor::new(&mut buf))
            .unwrap();

        println!("Output:     {}", y.to_hex_string(Case::Lower));
        println!(
            "Proof blob: {}",
            [&y[..], &buf[..]].concat().to_hex_string(Case::Lower)
        );
        println!("VK (cmp):   {}", vk_compressed.to_hex_string(Case::Lower));
    }
}
