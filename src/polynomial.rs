use ark_ff::PrimeField;
use polynomial::multilinear::coefficient_form::CoeffMultilinearPolynomial;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomial;

/// Create a new polynomial, using coefficient form for inputs but evaluation form for processing
pub fn poly<F: PrimeField>(
    n_vars: usize,
    terms: Vec<(F, Vec<bool>)>,
) -> Result<MultiLinearPolynomial<F>, &'static str> {
    let evaluations = CoeffMultilinearPolynomial::new(n_vars as u32, terms)?.to_evaluation_form();
    Ok(MultiLinearPolynomial::new(n_vars, evaluations)?)
}

/// Generate the univariate polynomial by summing over the boolean hypercube
pub fn skip_one_and_sum_over_boolean_hypercube<F: PrimeField>(
    poly: &MultiLinearPolynomial<F>,
) -> Vec<F> {
    let evaluations = poly.evaluation_slice();
    let (left_half, right_half) = evaluations.split_at(evaluations.len() / 2);
    let f_0 = left_half.iter().sum();
    let f_1 = right_half.iter().sum();
    vec![f_0, f_1]
}
