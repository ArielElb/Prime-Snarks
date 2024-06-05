use std::cmp;

use ark_ff::{PrimeField, BigInteger};
use ark_r1cs_std::{
    prelude::*,
    fields::fp::FpVar,
    eq::EqGadget,
};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef, ConstraintSynthesizer, SynthesisError};
use ark_bls12_381::Fq as F;
use rand::thread_rng;

fn Modulo_Checker<ConstraintF>(x: FpVar<ConstraintF>, q: FpVar<ConstraintF>, n: FpVar<ConstraintF>, y: FpVar<ConstraintF>) -> Result<(), SynthesisError>
where
    ConstraintF: PrimeField,
    FpVar<ConstraintF>: EqGadget<ConstraintF>,
{
    let mul = q.clone() * n.clone();
    let val = mul + y.clone();
    x.is_eq(&val)?.enforce_equal(&Boolean::TRUE)?;
    Ok(())
}
fn Modulo_Helper<ConstraintF>(x: FpVar<ConstraintF>, q: FpVar<ConstraintF>, n: FpVar<ConstraintF>, y: FpVar<ConstraintF>) 
where
    ConstraintF: PrimeField,
    FpVar<ConstraintF>: EqGadget<ConstraintF>,
{
    let cs = ConstraintSystem::<ConstraintF>::new_ref();
    let n_var = FpVar::new_input(cs.clone(), || Ok(n)).unwrap();
    let n_var = FpVar::new_input(cs.clone(), || Ok(n)).unwrap();
    let y_var = FpVar::new_input(cs.clone(), || Ok(y)).unwrap();
}
pub struct ModuloCircuit<ConstraintF: PrimeField> {
    x: FpVar<ConstraintF>, // input
    q: FpVar<ConstraintF>, // division result
    n: FpVar<ConstraintF>, // size of n modulo
    y: FpVar<ConstraintF>, // output
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for ModuloCircuit<ConstraintF> {
    fn generate_constraints(self, cs: ConstraintSystemRef<ConstraintF>) -> Result<(), SynthesisError> {
        let x = self.x;
        let q = self.q;
        let n = self.n;
        let y = self.y;

        // Enforce that x == q * n + y
        let mul = q.clone() * n.clone();
        let val = mul + y.clone();
        x.enforce_equal(&val)?;
        println!("q is {}", q.value().unwrap());
        println!("n is {}", n.value().unwrap());
        println!("y is {}", y.value().unwrap());

        println!("x is {}", x.value().unwrap());
        println!("v is {}", val.value().unwrap());
        // Enforce that y < n
        //let less_than = y.is_cmp(&n, std::cmp::Ordering::Less,true)?;
        //less_than.enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Bls12_381;
    use ark_ff::{UniformRand, Zero};
    use ark_groth16::Groth16;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_snark::SNARK;

    #[test]
    fn modulo_tester() {
        println!("hereeee");
        // Create a random field element
        let mut rng = thread_rng();
        let mut x = F::rand(&mut rng);
        let mut temp = x;
        let n = F::rand(&mut rng);

        // Ensure x and n are valid
        //assert!(x < F::MODULUS.into());
        //assert!(n < F::MODULUS.into());
        let mut q = F::from(0);
        let one = n / n;
        let mut y = x;
        while temp >= n {
            temp = temp - n;
            y = temp;
            q = q + one;
        }
        // Calculate q and y
        // Create FpVars for the circuit
        let cs = ConstraintSystem::<F>::new_ref();
        let x_var: FpVar<_> = FpVar::new_input(cs.clone(), || Ok(x)).unwrap();
        let q_var = FpVar::new_input(cs.clone(), || Ok(q)).unwrap();
        let n_var = FpVar::new_input(cs.clone(), || Ok(n)).unwrap();
        let y_var = FpVar::new_input(cs.clone(), || Ok(y)).unwrap();

        // Call Modulo_Checker function
        let res = Modulo_Checker(x_var.clone(), q_var.clone(), n_var.clone(), x_var.clone()).unwrap();
        assert!(true);
        // Instantiate the circuit
        let circuit = ModuloCircuit {
            x: x_var.clone(),
            q: q_var.clone(),
            n: n_var.clone(),
            y: y_var.clone(),
        };

        // Generate constraints
        circuit.generate_constraints(cs.clone()).unwrap();

        // Check if the constraints are satisfied
        assert!(cs.is_satisfied().unwrap());
        println!("Circuit is satisfied");
    }
}
