use wasm_zkp_challenge::pairing::{generate_pairing_inputs, compute_billinearity};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_pairing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_pairing");
    for size in [2, 4, 6, 8].iter() {
        let (g1_rand_vec, g2_rand_vec) = generate_pairing_inputs(1<<size);
        let g1_rand_vec = black_box(g1_rand_vec);
        let g2_rand_vec = black_box(g2_rand_vec);
        let input = (g1_rand_vec, g2_rand_vec);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("input vector length: 2^{}", size)),
            &input,
            |b, input| {
                b.iter(|| {
                    compute_billinearity(input.0.clone(), input.1.clone());
                })
            }
        );
    }
}

criterion_group!(benches, bench_pairing);
criterion_main!(benches);
