#![cfg(feature = "vdf")]
#![allow(dead_code)]

use blake3::hash;
use num_bigint::{BigUint, ToBigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};

static N: Lazy<BigUint> = Lazy::new(|| {
    BigUint::parse_bytes(
        b"13F5814B161391A553A80335D9109B948B8E81A693F4E564F73EA4BA388812BC\
599815F49CA9B5A97A518EC823B334F95835E4AAE0447C9EA9A01A64D6DAD0CE\
C38322B458BD665ABA12124CC751F3ABD3E44DC72E56408EAECE43860CCD2985\
5EE4EDCD454FF2CC8C02CA14F0B823AC4178F4F9E07BEB45F96BCBC1DFCB21D4\
4BFDAF7E61D7A3E60046D8F7F42F07B8B986D0B4205C970AE45BBCB976BFBB55\
CBCEBC00FF3B8545611C01F7C940148089A981AAD7755E23C0B7A80EEA2C9151\
DFA50EF50A041CE507A362D5EC6C58E9FD578243AF61F1FC0B6983A9FF9B1C51\
B83222354F1DA3DFAE04E5BB3BAD9D4A51F6FDF0CF325F9BD28FBA7A332A8FE5",
        16,
    )
    .expect("invalid N")
});

pub const L_BITS: usize = 128;

pub const OUTPUT_LEN: usize = 32;

fn h_to_group(msg: &[u8]) -> BigUint {
    let mut ctr = 0u8;
    loop {
        let mut h = hash(&[msg, &[ctr]].concat()).as_bytes().to_vec();
        h.reverse();
        let x = BigUint::from_bytes_be(&h) % &*N;
        if !x.is_zero() && x.gcd(&*N).is_one() {
            return x;
        }
        ctr = ctr.wrapping_add(1);
    }
}

fn derive_prime(seed: &[u8], t: u64) -> BigUint {
    let mut d = Sha256::new();
    d.update(seed);
    d.update(&t.to_be_bytes());

    loop {
        let mut cand = d.clone().finalize().to_vec();

        cand.truncate(16);

        cand[0] |= 0b1000_0001;
        cand[15] |= 0b0000_0001;

        let p = BigUint::from_bytes_be(&cand);
        if primal_check(&p) {
            return p;
        }
        d.update(&[0x01]);
    }
}

fn primal_check(p: &BigUint) -> bool {
    use num_bigint::BigUint as B;
    const BASES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

    if p.is_even() {
        return *p == B::from(2u8);
    }
    let one = B::one();
    let two = &one + &one;
    let p1 = p - &one;

    let s = p1.trailing_zeros().unwrap();
    let d = &p1 >> s;

    'basis: for &a_u64 in BASES {
        if p <= &B::from(a_u64) {
            continue;
        }
        let mut x = a_u64.to_biguint().unwrap().modpow(&d, p);
        if x == one || x == p1 {
            continue 'basis;
        }
        for _ in 1..s {
            x = x.modpow(&two, p);
            if x == p1 {
                continue 'basis;
            }
        }
        return false;
    }
    true
}

pub fn prove_vdf(seed: &[u8], t: u64) -> ([u8; OUTPUT_LEN], BigUint) {
    let x = h_to_group(seed);

    let mut y = x.clone();
    let two = 2u32.to_biguint().unwrap();
    for _ in 0..t {
        y = y.modpow(&two, &*N);
    }

    let ell = derive_prime(seed, t);

    let two_pow_t = BigUint::one() << t;
    let (q, _r) = two_pow_t.div_mod_floor(&ell);

    let pi = x.modpow(&q, &*N);

    let mut out = [0u8; OUTPUT_LEN];
    let y_hash = hash(&y.to_bytes_be());
    out.copy_from_slice(y_hash.as_bytes());

    (out, pi)
}

pub fn verify_vdf(seed: &[u8], t: u64, out: &[u8; OUTPUT_LEN], pi: &BigUint) -> bool {
    let x = h_to_group(seed);
    let ell = derive_prime(seed, t);

    let two_pow_t = BigUint::one() << t;
    let r = &two_pow_t % &ell;

    let lhs = pi.modpow(&ell, &*N);
    let xr = x.modpow(&r, &*N);
    let y_full = (lhs * xr) % &*N;

    let y_hash = hash(&y_full.to_bytes_be());
    out == y_hash.as_bytes()
}

pub fn compress_vdf(y: &BigUint) -> [u8; OUTPUT_LEN] {
    let mut out = [0u8; OUTPUT_LEN];
    out.copy_from_slice(hash(&y.to_bytes_be()).as_bytes());
    out
}

#[derive(Clone, Debug)]
pub struct VdfState {
    acc: BigUint,
}

impl VdfState {
    pub fn new(seed: &[u8]) -> Self {
        Self {
            acc: h_to_group(seed),
        }
    }

    pub fn output(&self) -> [u8; OUTPUT_LEN] {
        compress_vdf(&self.acc)
    }
}

pub fn prove_one_step(state: &mut VdfState) {
    static TWO: Lazy<BigUint> = Lazy::new(|| 2u32.to_biguint().unwrap());

    state.acc = state.acc.modpow(&TWO, &*N);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_small() {
        let seed = b"demo-seed";
        for t in 0u64..=20 {
            let (y, pi) = prove_vdf(seed, t);
            assert!(verify_vdf(seed, t, &y, &pi));
        }
    }
}
