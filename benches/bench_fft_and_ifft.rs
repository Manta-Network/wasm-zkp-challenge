// use wasm_zkp_challenge;
use wasm_zkp_challenge::fft::{generate_random_evaluation, compute_fft_and_ifft};
// use wasm_zkp_challenge::pairing::{generate_pairing_inputs, compute_billinearity};
// use wasm_zkp_challenge::msm::{generate_msm_inputs, compute_msm};
use ark_ec::{AffineCurve, ProjectiveCurve, short_weierstrass_jacobian::GroupProjective, short_weierstrass_jacobian::GroupAffine, bls12::Bls12Parameters};
use ark_ff::{prelude::*, Fp384};
use ark_std::{vec::Vec, UniformRand, rand::Rng};
use ark_bls12_381::{G1Affine, FqParameters};


use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use std::iter;

use criterion::Throughput;

fn from_elem(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("from_elem");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| iter::repeat(0u8).take(size).collect::<Vec<_>>());
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
