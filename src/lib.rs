use ark_bls12_381::G1Affine;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::PrimeField;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod msm;

#[wasm_bindgen]
pub struct PointVectorInput {
    point_vec: Vec<<<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::Affine>,
}

#[wasm_bindgen]
impl PointVectorInput {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        Self {
            point_vec: msm::generate_msm_inputs::<G1Affine>(size).0,
        }
    }
}

#[wasm_bindgen]
pub struct ScalarVectorInput {
    scalar_vec: Vec<<<G1Affine as AffineCurve>::ScalarField as PrimeField>::BigInt>,
}

#[wasm_bindgen]
impl ScalarVectorInput {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        Self {
            scalar_vec: msm::generate_msm_inputs::<G1Affine>(size).1,
        }
    }
}

#[wasm_bindgen]
pub fn compute_msm(point_vec: &PointVectorInput, scalar_vec: &ScalarVectorInput) {
    let _ = msm::compute_msm::<G1Affine>(&point_vec.point_vec, &scalar_vec.scalar_vec);
}
