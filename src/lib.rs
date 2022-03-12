#![feature(test)]

mod bench;
pub mod fft;
pub mod msm;
pub mod pairing;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bench_fft_and_ifft() {
    let input_domain_dim = 14;
    let output_domain_dim = 18;
    let (rand_evaluation_domain, output_domain) = fft::generate_random_evaluation(input_domain_dim, output_domain_dim);

    fft::compute_fft_and_ifft(rand_evaluation_domain.clone(), output_domain);
}

#[wasm_bindgen]
pub fn bench_msm() {
    let size = 1<<14;
    let (point_vec, scalar_vec) = msm::generate_msm_inputs(size);
    msm::compute_msm(point_vec, scalar_vec);
}

#[wasm_bindgen]
pub fn bench_pairing() {
    let size = 1<<2;
    let (g1_rand_vec, g2_rand_vec) = pairing::generate_pairing_inputs(size);
    pairing::compute_billinearity(g1_rand_vec, g2_rand_vec);
}