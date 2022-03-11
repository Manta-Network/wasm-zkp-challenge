#![feature(test)]

mod bench;
pub mod fft;

use wasm_bindgen::prelude::*;

//#[wasm_bindgen]
//extern {
//    pub fn alert(s: &str);
//}

// #[wasm_bindgen]
// pub fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 0,
//         1 => 1,
//         x => fibonacci(x - 1) + fibonacci(x - 2),
//     }
// }

// #[wasm_bindgen]
// pub fn pedersen_prove() {
//     prover::pedersen_prover();
// }

// #[wasm_bindgen]
// pub fn blake_prove() {
//     prover::blake_prover();
// }

#[wasm_bindgen]
pub fn bench_fft_and_ifft() {
    let input_domain_dim = 20;
    let output_domain_dim = 20;
    let (rand_evaluation_domain, output_domain) = fft::generate_random_evaluation(input_domain_dim, output_domain_dim);

    fft::compute_fft_and_ifft(rand_evaluation_domain.clone(), output_domain);
}