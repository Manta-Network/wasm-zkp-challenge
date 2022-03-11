#![feature(test)]

mod bench;
pub mod fft;
pub mod msm;
pub mod pairing;

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