extern crate test;

#[cfg(test)]
mod tests{
    use crate::fft::{generate_random_evaluation, compute_fft_and_ifft};
    use crate::pairing::{generate_pairing_inputs, compute_billinearity};
    use crate::msm::{generate_msm_inputs, compute_msm};
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_fft_and_ifft(b: &mut Bencher) {
        let input_domain_dim = 15;
        let output_domain_dim = 16;
        let (rand_evaluation_domain, output_domain) = generate_random_evaluation(input_domain_dim, output_domain_dim);

        let rand_evaluation_domain = black_box(rand_evaluation_domain);
        let output_domain = black_box(output_domain);

        b.iter( || {
            compute_fft_and_ifft(rand_evaluation_domain.clone(), output_domain);
        });
    }

    #[bench]
    fn bench_pairing(b: &mut Bencher) {
        let size = 1<<6;
        let (g1_rand_vec, g2_rand_vec) = generate_pairing_inputs(size);
        
        let g1_rand_vec = black_box(g1_rand_vec);
        let g2_rand_vec = black_box(g2_rand_vec);

        b.iter(|| {
            compute_billinearity(g1_rand_vec.clone(), g2_rand_vec.clone());
        });
    }

    #[bench]
    fn bench_msm(b: &mut Bencher) {
        let size = 1<<10;
        let (point_vec, scalar_vec) = generate_msm_inputs(size);

        let point_vec = black_box(point_vec);
        let scalar_vec = black_box(scalar_vec);
        
        b.iter( || {
            compute_msm(point_vec.clone(), scalar_vec.clone());
        });
    }
}