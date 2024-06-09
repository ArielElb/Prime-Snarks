use ark_ff::{PrimeField, Field, UniformRand};
use ark_r1cs_std::{
    prelude::*,
    fields::fp::FpVar,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError};
use ark_bls12_381::Fq as F;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey, Proof};
use ark_std::rand::thread_rng;
use ark_crypto_primitives::snark::SNARK;
use num_bigint::BigUint;

#[derive(Clone)]
pub struct ExponentiationCircuit<ConstraintF: PrimeField> {
    base: FpVar<ConstraintF>,
    pow: u64,
    answer: FpVar<ConstraintF>,
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for ExponentiationCircuit<ConstraintF> {
    fn generate_constraints(self, cs: ConstraintSystemRef<ConstraintF>) -> Result<(), SynthesisError> {
        let base = self.base;
        let pow = self.pow;
        let answer = self.answer;

        // Compute base^pow
        let mut result = FpVar::one();
        let mut prev = result.clone();
        let power = [pow];
        //let var = FpVar::<F>::new_constant(cs.clone(), || self.base.ok_or(SynthesisError::AssignmentMissing))?;
        for i in 0..pow{
            result = result*base.clone();
            //result.enforce_equal(&(&prev*&base));
        }
        //let result = base.value().unwrap().pow(&power);
        // Enforce that result == answer
        result.enforce_equal(&answer)?;
        Ok(())
    }
}
fn exp_ver(base_val:BigUint,power: u64,ans_val:BigUint) {
    let mut rng = thread_rng();
    let base: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FqConfig, 6>, 6> = F::from(base_val);
    let pow = F::from(power); // Let's use a small exponent for simplicity
    let answer = F::from(ans_val);
    let res = F::from(1);

    let cs = ConstraintSystem::<F>::new_ref();

    // Create FpVars for the circuit
    //let base_var = FpVar::new_input(cs.clone(), || Ok(base)).unwrap();
    let base_var: FpVar<_> = FpVar::new_input(cs.clone(), || Ok(base)).unwrap();
    let pow_var = FpVar::new_input(cs.clone(), || Ok(pow)).unwrap();
    let mut answer_var = FpVar::new_input(cs.clone(), || Ok(answer)).unwrap();
    let result = FpVar::new_input(cs.clone(), || Ok(res)).unwrap();
    // Instantiate the circuit
    //let circuit = ExponentiationCircuit {
    //    base: base_var.clone(),
    //    pow: 3,
    //    answer: answer_var,
    //};
    
    for i in 0..power{
        answer_var = result.clone()*base_var.clone();
    }
    //let result = base.value().unwrap().pow(&power);
    // Enforce that result == answer
    result.enforce_equal(&answer_var);
    //circuit.generate_constraints(cs.clone()).unwrap();
    assert!(cs.is_satisfied().unwrap());
    //Ok(())
    
    }

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Bls12_381;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn exponentiation_verifier() {
        let mut rng = thread_rng();
        let base = F::rand(&mut rng);
        let pow = F::from(3u64); // Let's use a small exponent for simplicity
        let answer = base.pow([3u64]);//F::rand(&mut rng);

        // Ensure the answer is correct
        assert_eq!(answer, base.pow([3u64]));

        // Create ConstraintSystem
        let cs = ConstraintSystem::<F>::new_ref();

        // Create FpVars for the circuit
        let base_var = FpVar::new_input(cs.clone(), || Ok(base)).unwrap();
        let base_var: FpVar<_> = FpVar::new_input(cs.clone(), || Ok(base)).unwrap();
        let pow_var = FpVar::new_input(cs.clone(), || Ok(pow)).unwrap();
        let answer_var = FpVar::new_input(cs.clone(), || Ok(answer)).unwrap();

        // Instantiate the circuit
        let circuit = ExponentiationCircuit {
            base: base_var.clone(),
            pow: 3,
            answer: answer_var,
        };
        print!("fpvar is {}",base_var.value().unwrap());

        print!("fpvar is {}",base_var.value().unwrap());

        circuit.generate_constraints(cs.clone()).unwrap();
        //
        //// Check if the constraints are satisfied
        //let mut inputs = Vec::new();
        //inputs.push(base_var);
        //inputs.push(5);
        //inputs.push(answer_var);
        //let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();
        //let pvk: ark_groth16::PreparedVerifyingKey<ark_ec::bls12::Bls12<ark_bls12_381::Config>> = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
        assert!(cs.is_satisfied().unwrap());
        
        println!("Exponentiation circuit is verified");
    }
}
