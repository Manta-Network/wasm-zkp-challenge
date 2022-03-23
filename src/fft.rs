// use ark_poly;
use ark_poly::{evaluations::univariate::Evaluations, EvaluationDomain, Radix2EvaluationDomain};
use ark_bls12_381::Fr;
use ark_std::test_rng;
use ark_std::UniformRand;


pub fn generate_random_evaluation(
    input_domain_dim: usize,
    output_domain_dim: usize,
) -> (Evaluations<Fr,Radix2EvaluationDomain<Fr>>, Radix2EvaluationDomain<Fr>)
{
    assert!(input_domain_dim<=output_domain_dim, "input_domain_dim should be less or equal to output_domain_dim");
    // Generate domain
    let input_domain = Radix2EvaluationDomain::<Fr>::new(1<<input_domain_dim).unwrap();
    let output_domain = Radix2EvaluationDomain::<Fr>::new(1<<output_domain_dim).unwrap();

    // Generate random vector
    let mut rng = test_rng();
    let degree = 1<<input_domain_dim-1;
    let mut rand_vec: Vec<Fr> = Vec::with_capacity(degree);
    for _i in 0..degree {
        rand_vec.push(Fr::rand(&mut rng));
    }

    let rand_evaluation_domain = Evaluations::from_vec_and_domain(rand_vec, input_domain);

    (rand_evaluation_domain, output_domain)
}

pub fn compute_fft_and_ifft(
    rand_evaluation_domain: Evaluations<Fr, Radix2EvaluationDomain<Fr>>,
    output_domain: Radix2EvaluationDomain<Fr>,
) -> Vec<Fr> {
    // Run ifft to get coefficients
    let dense_poly = rand_evaluation_domain.interpolate();

    // Run fft to evaluate the polynomial
    let output_evaluation = dense_poly.evaluate_over_domain(output_domain);

    // Return the output vector
    output_evaluation.evals
}