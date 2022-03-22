// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// use crate::fft::{generate_random_evaluation, compute_fft_and_ifft};
// use crate::pairing::{generate_pairing_inputs, compute_billinearity};
// use crate::msm::{generate_msm_inputs, compute_msm};
// use super::*;
// use ark_ec::{AffineCurve, ProjectiveCurve, short_weierstrass_jacobian::GroupProjective, short_weierstrass_jacobian::GroupAffine, bls12::Bls12Parameters};
// use test::{Bencher, black_box};
// use ark_ff::{prelude::*, Fp384};
// use ark_std::{vec::Vec, UniformRand, rand::Rng};
// use ark_bls12_381::{G1Affine, FqParameters};

// fn bench_fft_and_ifft(c: &mut Criterion) {
//     let input_domain_dim = 14;
//     let output_domain_dim = 16;
//     let (rand_evaluation_domain, output_domain) = generate_random_evaluation(input_domain_dim, output_domain_dim);

//     let rand_evaluation_domain = black_box(rand_evaluation_domain);
//     let output_domain = black_box(output_domain);

//     b.iter( || {
//         compute_fft_and_ifft(rand_evaluation_domain.clone(), output_domain);
//     });
// }
// use criterion::BenchmarkId;
// use criterion::Criterion;
// use criterion::{criterion_group, criterion_main};

// fn do_something(size: usize) {
//     // Do something with the size
// }

// fn from_elem(c: &mut Criterion) {
//     let size: usize = 1024;

//     c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
//         b.iter(|| do_something(s));
//     });
// }

// criterion_group!(benches, from_elem);
// criterion_main!(benches);