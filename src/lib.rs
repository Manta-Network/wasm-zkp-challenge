pub mod fft;
pub mod msm;
pub mod pairing;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bench_fft_and_ifft(input_domain_dim: usize, output_domain_dim: usize) {
    let (rand_evaluation_domain, output_domain) = fft::generate_random_evaluation(input_domain_dim, output_domain_dim);

    fft::compute_fft_and_ifft(rand_evaluation_domain, output_domain);
}

#[wasm_bindgen]
pub fn bench_msm(size: usize) {
    let (point_vec, scalar_vec) = msm::generate_msm_inputs(1<<size);
    msm::compute_msm(point_vec, scalar_vec);
}

#[wasm_bindgen]
pub fn bench_pairing(size: usize) {
    let (g1_rand_vec, g2_rand_vec) = pairing::generate_pairing_inputs(1<<size);
    pairing::compute_billinearity(g1_rand_vec, g2_rand_vec);
}