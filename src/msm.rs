// use ark_bls12_381::G1Affine;
use ark_ec::{AffineCurve, ProjectiveCurve,msm::VariableBase};
// use ark_ff::{UniformRand};
use ark_ff::fields::PrimeField;

use ark_algebra_test_templates::{
    curves::*, fields::*, generate_field_test, generate_g1_test, groups::*, msm::*,
};

// use ark_ec::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{Field, One, SquareRootField, UniformRand, Zero};

use ark_bls12_381::{g1, Fq, Fq2, Fq6, Fr, G1Affine, G1Projective};
use ark_std::{
    ops::{AddAssign, MulAssign, SubAssign},
    rand::Rng,
    test_rng,
};

pub fn test_var_base_msm<G: AffineCurve>() {
    const SAMPLES: usize = 1 << 10;

    let mut rng = ark_std::test_rng();
    // TODO: Not sure why into_bigint() is not found here.
    let v = (0..SAMPLES - 1)
        .map(|_| G::ScalarField::rand(&mut rng).into_bigint())
        .collect::<Vec<_>>();
    let g = (0..SAMPLES)
        .map(|_| G::Projective::rand(&mut rng))
        .collect::<Vec<_>>();
    let g = <G::Projective as ProjectiveCurve>::batch_normalization_into_affine(&g);

    let fast = VariableBase::msm(g.as_slice(), v.as_slice());
}

#[test]
fn test() {
    // TODO: Not sure why G1Affine cannot be used here.
    test_var_base_msm::<G1Affine>();
}

// // generate_field_test!(bls12_381; fq2; fq6; mont(6, 4); );
// generate_g1_test!(bls12_381; curve_tests; sw_tests;);



// pub fn generate_random_input<G: AffineCurve>()
// ->(Vec<_>, Vec<_>)
// {
//     const SAMPLES: usize = 1 << 10;
//     let mut rng = ark_std::test_rng();

//     let v = (0..SAMPLES-1)
//         .map(|_| G::ScalarField::rand(&mut rng).into_bigint())
//         .collect::<Vec<_>>();
//     let g = (0..SAMPLES)
//         .map(|_| G::Projective::rand(&mut rng))
//         .collect::<Vec<_>>();
//     let g = <G::Projective as ProjectiveCurve>::batch_normalization_into_affine(&g);

//     (g, v)
// }



// pub fn compute_msm() {

// }



