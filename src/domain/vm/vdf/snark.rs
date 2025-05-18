#![cfg(feature = "zk-snark")]
#![forbid(unsafe_code)]

use anyhow::Result;
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
use ark_std::{
    rand::{rngs::StdRng, SeedableRng},
    vec::Vec,
};
use once_cell::sync::Lazy;

pub const OUTPUT_LEN: usize = 32;

pub fn build_poseidon_params() -> PoseidonConfig<Fr> {
    const WIDTH: usize = 3; // rate = 2, capacity = 1
    const FULL_ROUNDS: usize = 8;
    const PARTIAL_ROUNDS: usize = 57;
    const ALPHA: u64 = 5; // S-box x^5

    let total_rounds = FULL_ROUNDS + PARTIAL_ROUNDS;
    let mut rng = StdRng::seed_from_u64(0xA11CE);

    let ark: Vec<Vec<Fr>> = (0..total_rounds)
        .map(|_| (0..WIDTH).map(|_| Fr::rand(&mut rng)).collect())
        .collect();

    let mds: Vec<Vec<Fr>> = (0..WIDTH)
        .map(|_| (0..WIDTH).map(|_| Fr::rand(&mut rng)).collect())
        .collect();

    PoseidonConfig {
        full_rounds: FULL_ROUNDS,
        partial_rounds: PARTIAL_ROUNDS,
        alpha: ALPHA,
        ark,
        mds,
        rate: 2,
        capacity: 1,
    }
}

static POSEIDON_PARAMS: Lazy<PoseidonConfig<Fr>> = Lazy::new(build_poseidon_params);

pub type Output = [u8; 32];

#[inline]
pub fn fr_from_bytes(bytes: &[u8]) -> Fr {
    Fr::from_le_bytes_mod_order(bytes)
}
#[inline]
pub fn fr_to_bytes(fr: &Fr) -> Output {
    let mut out = [0u8; 32];
    let limbs = fr.into_bigint().to_bytes_le();
    out[..limbs.len()].copy_from_slice(&limbs);
    out
}

#[inline]
fn poseidon_hash_native(inputs: &[Fr]) -> Fr {
    PoseidonCRH::<Fr>::evaluate(&POSEIDON_PARAMS, inputs).expect("Poseidon hash failed")
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

    let mut rng = StdRng::seed_from_u64(42);
    let pk = Groth16::<Bls12_381>::generate_random_parameters_with_reduction(dummy, &mut rng)?;
    Ok((pk.clone(), pk.vk))
}

pub fn expected_output(seed: &Fr, t: u64) -> Fr {
    let mut cur = *seed;
    for _ in 0..t {
        cur = poseidon_hash_native(&[cur]);
    }
    cur
}

pub fn prove(
    seed_bytes: &[u8],
    t: u64,
) -> Result<(Output, Proof<Bls12_381>, VerifyingKey<Bls12_381>)> {
    let seed_fr = fr_from_bytes(seed_bytes);
    let y_fr = expected_output(&seed_fr, t);
    let pub_hash_fr = poseidon_hash_native(&[seed_fr, y_fr]);

    let circ = PoseidonIterCircuit {
        init: seed_fr,
        iterations: t,
        pub_hash_fr,
    };

    let (pk, vk) = gen_keys(t)?;

    let mut rng = StdRng::seed_from_u64(99);
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(circ, &pk, &mut rng)?;

    Ok((fr_to_bytes(&y_fr), proof, vk))
}

pub fn verify(
    seed_bytes: &[u8],
    y_bytes: &Output,
    proof: &Proof<Bls12_381>,
    vk: &VerifyingKey<Bls12_381>,
) -> bool {
    let seed_fr = fr_from_bytes(seed_bytes);
    let y_fr = fr_from_bytes(y_bytes);
    let pub_fr = poseidon_hash_native(&[seed_fr, y_fr]);

    let pvk = prepare_verifying_key(vk);
    let prepared_inputs =
        Groth16::<Bls12_381>::prepare_inputs(&pvk, &[pub_fr]).expect("prepare_inputs failed");

    Groth16::<Bls12_381>::verify_proof_with_prepared_inputs(&pvk, proof, &prepared_inputs)
        .unwrap_or(false)
}

pub fn poseidon_array(seed: &[u8]) -> Output {
    let seed_fr = fr_from_bytes(seed);
    let y_fr = poseidon_hash_native(&[seed_fr]);
    fr_to_bytes(&y_fr)
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
        assert!(verify(seed, &y, &proof, &vk));

        let mut buf = Vec::with_capacity(proof.serialized_size(Compress::Yes));
        let mut vk_compressed = Vec::with_capacity(vk.serialized_size(Compress::Yes));

        vk.serialize_compressed(&mut Cursor::new(&mut vk_compressed))
            .unwrap();

        proof
            .serialize_compressed(&mut Cursor::new(&mut buf))
            .unwrap();

        // put y in the front
        let output_proof = [&y[..], &buf[..]].concat();

        println!("Output: {:?}", y.to_hex_string(Case::Lower));
        println!("Proof: {:?}", output_proof.to_hex_string(Case::Lower));
        println!(
            "Compressed vk: {:?}",
            vk_compressed.to_hex_string(Case::Lower)
        );
    }
}
