use ark_ec::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve, short_weierstrass_jacobian::GroupProjective};
use ark_bls12_381::{g1, g2, Bls12_381, Fq, Fq12, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective, Parameters};
use ark_ff::{
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    One, Zero,
};
use ark_std::rand::Rng;
use ark_std::test_rng;
use core::ops::{AddAssign, MulAssign};

pub fn generate_pairing_inputs(size: usize) 
-> (Vec<G1Projective>, Vec<G2Projective>)
{
    let mut rng = test_rng();
    let mut g1_rand_vec = Vec::with_capacity(size);
    let mut g2_rand_vec = Vec::with_capacity(size);
    
    for _i in 0..size {
        let a: G1Projective = rng.gen();
        let b: G2Projective = rng.gen();
        g1_rand_vec.push(a);
        g2_rand_vec.push(b);
    }

    (g1_rand_vec, g2_rand_vec)
}

pub fn compute_billinearity(g1_vector: Vec<G1Projective>, g2_vector: Vec<G2Projective>) {
    assert!(g1_vector.len() == g2_vector.len(), "Length of g1 vector and g2 vector should be the same");
    let size = g1_vector.len();
    let mut res_vec = Vec::with_capacity(size);
    for _i in 0..size {
        let res = Bls12_381::pairing(g1_vector[_i], g2_vector[_i]);
        res_vec.push(res);
    }
}

#[test]
fn test_bilinearity() {
    let size = 1<<6;
    let (g1_rand_vec, g2_rand_vec) = generate_pairing_inputs(size);
    compute_billinearity(g1_rand_vec, g2_rand_vec);
}
