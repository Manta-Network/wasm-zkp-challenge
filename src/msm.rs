use ark_ec::{msm::VariableBaseMSM, AffineCurve, ProjectiveCurve};
use ark_ff::{PrimeField, UniformRand, Zero};
use ark_bls12_381::G1Affine;
use crate::stream_pippenger::ChunkedPippenger;

pub fn generate_msm_inputs(size: usize)
-> (
    Vec<<<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::Affine>, 
    Vec<<<G1Affine as AffineCurve>::ScalarField as PrimeField>::BigInt>,
){
    let mut rng = ark_std::test_rng();

    let scalar_vec = (0..size - 1)
        .map(|_| <G1Affine as AffineCurve>::ScalarField::rand(&mut rng).into_repr())
        .collect::<Vec<_>>();
    let point_vec = (0..size)
        .map(|_| <G1Affine as AffineCurve>::Projective::rand(&mut rng))
        .collect::<Vec<_>>();
    let point_vec = <<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::batch_normalization_into_affine(&point_vec);

    (point_vec, scalar_vec)
}

/// Currently using Pippenger's algorithm for multi-scalar multiplication (MSM)
pub fn compute_msm(
    point_vec: Vec<<<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::Affine>,
    scalar_vec: Vec<<<G1Affine as AffineCurve>::ScalarField as PrimeField>::BigInt>,
) -> <G1Affine as AffineCurve>::Projective
{
    VariableBaseMSM::multi_scalar_mul(point_vec.as_slice(), scalar_vec.as_slice())
}

#[test]
fn test() {
    let size = 1<<10;
    let (point_vec, scalar_vec) = generate_msm_inputs(size);
    let res = compute_msm(point_vec, scalar_vec);
}

#[test]
fn test_pippenger_correctness() {
    let size = 1<<12;

    let (point_vec, scalar_vec) = generate_msm_inputs(size);

    let pippenger_msm = compute_msm(point_vec.clone(), scalar_vec.clone());

    let mut p = ChunkedPippenger::<G1Affine>::new(1 << 12);
    for (s, g) in scalar_vec.iter().zip(point_vec) {
        p.add(g, s);
    }
    let stream_pippenger_msm = p.finalize();

    assert_eq!(pippenger_msm.into_affine(), stream_pippenger_msm.into_affine());
}