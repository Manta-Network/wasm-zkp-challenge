extern crate test;

#[cfg(test)]
mod tests{
    use crate::fft::{generate_random_evaluation, compute_fft_and_ifft};
    use crate::pairing::{generate_pairing_inputs, compute_billinearity};
    use crate::msm::{generate_msm_inputs, compute_msm};
    use crate::pippenger_msm::{mixed_point_addition, batch_affine_point_addition, batch_affine_point_addition_pre_allocate};
    use crate::stream_pippenger::ChunkedPippenger;
    use super::*;
    use ark_ec::{AffineCurve, ProjectiveCurve, short_weierstrass_jacobian::GroupProjective, short_weierstrass_jacobian::GroupAffine, bls12::Bls12Parameters};
    use test::{Bencher, black_box};
    use ark_ff::{prelude::*, Fp384};
    use ark_std::{vec::Vec, UniformRand, rand::Rng};
    use ark_bls12_381::{G1Affine, FqParameters};

    #[bench]
    fn bench_fft_and_ifft(b: &mut Bencher) {
        let input_domain_dim = 14;
        let output_domain_dim = 18;
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
    fn bench_pippenger_msm(b: &mut Bencher) {
        let size = 1<<14;
        let (point_vec, scalar_vec) = generate_msm_inputs(size);

        let point_vec = black_box(point_vec);
        let scalar_vec = black_box(scalar_vec);
        
        b.iter( || {
            compute_msm(point_vec.clone(), scalar_vec.clone());
        });
    }

    #[bench]
    fn bench_strem_pippenger_msm(b: &mut Bencher) {
        let size = 1<<14;
        let (point_vec, scalar_vec) = generate_msm_inputs(size);

        let point_vec = black_box(point_vec);
        let scalar_vec = black_box(scalar_vec);
        
        b.iter( || {
            let mut p = ChunkedPippenger::<G1Affine>::new(1 << 12);
            for (s, g) in scalar_vec.iter().zip(point_vec.clone()) {
                p.add(g, s);
            }
            let stream_pippenger_msm = p.finalize();
        });
    }

    #[bench]
    fn bench_mixed_point_addition(b: &mut Bencher) {
        let size = 1<<18;

        let mut rng = ark_std::test_rng();
        let point_vec = (0..size)
            .map(|_| <G1Affine as AffineCurve>::Projective::rand(&mut rng))
            .collect::<Vec<_>>();
        let point_vec = <<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::batch_normalization_into_affine(&point_vec);
        let point_vec = black_box(point_vec);

        let first_index_vec: Vec<usize> = (0..size).map(|_| rng.gen_range(0..size)).collect();
        let second_index_vec: Vec<usize> = (0..size).map(|_| rng.gen_range(0..size)).collect();

        b.iter(|| {
            let mixed_point_addition_res = mixed_point_addition(point_vec.as_slice(), first_index_vec.as_slice(), second_index_vec.as_slice());
        });
    }

    #[bench]
    fn bench_batch_affine_point_addition(b: &mut Bencher) {
        let size = 1<<18;

        let mut rng = ark_std::test_rng();
        let point_vec = (0..size)
            .map(|_| <G1Affine as AffineCurve>::Projective::rand(&mut rng))
            .collect::<Vec<_>>();
        let point_vec = <<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::batch_normalization_into_affine(&point_vec);

        let first_index_vec: Vec<usize> = (0..size).map(|_| rng.gen_range(0..size)).collect();
        let second_index_vec: Vec<usize> = (0..size).map(|_| rng.gen_range(0..size)).collect();

        b.iter(|| {
            let batch_affine_point_addition_res = batch_affine_point_addition(point_vec.as_slice(), first_index_vec.as_slice(), second_index_vec.as_slice());
        });
    }

    #[bench]
    fn bench_batch_affine_point_addition_preallocate(b: &mut Bencher) {
        let size = 1<<14;

        let mut rng = ark_std::test_rng();
        let point_vec = (0..size)
            .map(|_| <G1Affine as AffineCurve>::Projective::rand(&mut rng))
            .collect::<Vec<_>>();
        let point_vec = <<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::batch_normalization_into_affine(&point_vec);
        let point_vec = black_box(point_vec);

        let first_index_vec: Vec<usize> = (0..size).map(|_| rng.gen_range(0..size)).collect();
        let second_index_vec: Vec<usize> = (0..size).map(|_| rng.gen_range(0..size)).collect();

        let mut a_vec = vec![<ark_ec::short_weierstrass_jacobian::GroupAffine<<ark_bls12_381::Parameters as Bls12Parameters>::G1Parameters> as AffineCurve>::BaseField::zero(); size];
        let mut d_vec = vec![<ark_ec::short_weierstrass_jacobian::GroupAffine<<ark_bls12_381::Parameters as Bls12Parameters>::G1Parameters> as AffineCurve>::BaseField::one(); size];
        let mut diff_vec = vec![<ark_ec::short_weierstrass_jacobian::GroupAffine<<ark_bls12_381::Parameters as Bls12Parameters>::G1Parameters> as AffineCurve>::BaseField::zero(); size];
        let mut e_vec = vec![<ark_ec::short_weierstrass_jacobian::GroupAffine<<ark_bls12_381::Parameters as Bls12Parameters>::G1Parameters> as AffineCurve>::BaseField::zero(); size];
        let mut r_vec = vec![<ark_ec::short_weierstrass_jacobian::GroupAffine<<ark_bls12_381::Parameters as Bls12Parameters>::G1Parameters> as AffineCurve>::BaseField::zero(); size];
        
        b.iter(|| {
            let batch_affine_point_addition_res = batch_affine_point_addition_pre_allocate(point_vec.as_slice(), first_index_vec.as_slice(), second_index_vec.as_slice(), a_vec.as_mut_slice(), d_vec.as_mut_slice(), diff_vec.as_mut_slice(), e_vec.as_mut_slice(), r_vec.as_mut_slice());
        });
    }
}