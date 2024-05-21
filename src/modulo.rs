use ark_ff::{PrimeField, BigInteger};
use ark_r1cs_std::{
    prelude::*,
    fields::fp::FpVar,
};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef, ConstraintSynthesizer, SynthesisError};
use ark_bls12_381::Fq as F;
use rand::thread_rng;

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
        println!("q is {}",q.value().unwrap());
        println!("n is {}",n.value().unwrap());
        println!("y is {}",y.value().unwrap());

        println!("x is {}",x.value().unwrap());
        println!("val is {}",val.value().unwrap());
        // Enforce that y < n
        let less_than = y.is_cmp(&n, std::cmp::Ordering::Less,false)?;
        //less_than.enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{UniformRand, Zero};
    use ark_relations::r1cs::ConstraintSystem;

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
        let mut q = n - n;
        let one = n/n;
        let mut y = x;
        while temp >= n {
            temp = temp - n;
            y = temp;
            q = q + one;
        }
        // Calculate q and y
        // Create FpVars for the circuit
        let cs = ConstraintSystem::<F>::new_ref();
        let x_var = FpVar::new_input(cs.clone(), || Ok(x)).unwrap();
        let q_var = FpVar::new_input(cs.clone(), || Ok(q)).unwrap();
        let n_var = FpVar::new_input(cs.clone(), || Ok(n)).unwrap();
        let y_var = FpVar::new_input(cs.clone(), || Ok(y)).unwrap();
        
        // Instantiate the circuit
        let circuit = ModuloCircuit {
            x: x_var,
            q: q_var,
            n: n_var,
            y: y_var,
        };

        // Generate constraints
        circuit.generate_constraints(cs.clone()).unwrap();
        //
        //// Check if the constraints are satisfied
        assert!(cs.is_satisfied().unwrap());

        println!("Circuit is satisfied");
    }
}
