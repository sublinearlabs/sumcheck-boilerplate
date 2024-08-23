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
