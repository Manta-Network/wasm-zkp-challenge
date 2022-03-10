extern crate test;

#[cfg(test)]
mod tests{
    use crate::fft::{generate_random_evaluation, compute_fft_and_ifft};

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fft_and_ifft(b: &mut Bencher) {
        let input_domain_dim = 15;
        let output_domain_dim = 16;
        let (rand_evaluation_domain, output_domain) = generate_random_evaluation(input_domain_dim, output_domain_dim);

        b.iter( || {
            compute_fft_and_ifft(rand_evaluation_domain.clone(), output_domain);
        });
    }
}