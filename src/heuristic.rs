use std::cmp;
use std::borrow::Borrow;
use ark_ff::{BigInt, BigInteger, PrimeField};
use ark_r1cs_std::{
    prelude::*,
    fields::fp::FpVar,
    eq::EqGadget,
};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef, ConstraintSynthesizer, SynthesisError};
use ark_bls12_381::Fq as F;
use num_bigint::{BigUint, UniformBigUint};
use rand::thread_rng;

fn fermat_test(num: BigUint){
    let cs = ConstraintSystem::<F>::new_ref();
    let num_val = F::from(num);
    let num_var: FpVar<_> = FpVar::new_input(cs.clone(), || Ok(num_val)).unwrap();


}