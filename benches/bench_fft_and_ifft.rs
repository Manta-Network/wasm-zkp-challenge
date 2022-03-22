use wasm_zkp_challenge::fft::{generate_random_evaluation, compute_fft_and_ifft};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_fft_and_ifft(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_fft_and_ifft");
    for input_domain_dim in [14, 16, 18, 20].iter() {
        for output_domain_dim in [input_domain_dim+1, input_domain_dim+2, input_domain_dim+3, input_domain_dim+4].iter() {
            if (*input_domain_dim>14) && (*output_domain_dim>*input_domain_dim+1){
                continue;
            }
            let (rand_evaluation_domain, output_domain) = generate_random_evaluation(*input_domain_dim, *output_domain_dim);
            let rand_evaluation_domain = black_box(rand_evaluation_domain);
            let output_domain = black_box(output_domain);
            let input = (rand_evaluation_domain, output_domain);

            group.bench_with_input(
                BenchmarkId::from_parameter(format!("input_domain_size: 2^{}, output_domain_size: 2^{}", input_domain_dim, output_domain_dim)),
                &input,
                |b, input| {
                    b.iter(|| {
                        compute_fft_and_ifft(input.0.clone(), input.1);
                    })
                }
            );
        }
    }
}

criterion_group!(benches, bench_fft_and_ifft);
criterion_main!(benches);
