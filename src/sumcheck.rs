use ark_ff::{BigInteger, PrimeField};
use polynomial::multilinear::evaluation_form::MultiLinearPolynomial;
use polynomial::univariate_poly::UnivariatePolynomial;
use transcript::Transcript;

pub struct SumcheckProof<F: PrimeField> {
    sum: F,
    round_polys: Vec<Vec<F>>,
}

fn prove<F: PrimeField>(poly: &MultiLinearPolynomial<F>, sum: F) -> SumcheckProof<F> {
    // for each variable
    //  generate round poly
    //  get challenge
    //  update poly by partially evaluating at the challenge
    unimplemented!()
}

fn verify<F: PrimeField>(
    poly: &MultiLinearPolynomial<F>,
    proof: &SumcheckProof<F>,
) -> Result<bool, &'static str> {
    if proof.round_polys.len() != poly.n_vars() {
        return Err("invalid proof: require 1 round poly for each variable in poly");
    }

    let mut claimed_sum = proof.sum;
    let mut challenges = vec![];

    // initialize the transcript
    let mut transcript = Transcript::new();

    // add the public inputs to the transcript
    transcript.append(poly.to_bytes().as_slice());
    transcript.append(proof.sum.into_bigint().to_bytes_be().as_slice());

    for round_poly in &proof.round_polys {
        // append the round poly to the transcript
        transcript.append(field_elements_to_bytes(round_poly.as_slice()).as_slice());
        let round_poly_as_univariate = UnivariatePolynomial::interpolate(round_poly.clone());

        // assert that p(0) + p(1) = sum
        let p_0 = round_poly_as_univariate.evaluate(&F::ZERO);
        let p_1 = round_poly_as_univariate.evaluate(&F::ONE);

        if claimed_sum != (p_0 + p_1) {
            return Err("verifier check failed: claimed_sum != p(0) + p(1)");
        }

        // sample challenge and update the claimed sum for next round
        let challenge = transcript.sample_field_element::<F>();
        claimed_sum = round_poly_as_univariate.evaluate(&challenge);
        challenges.push(challenge);
    }

    // perform oracle check
    let initial_poly_eval_at_challenge = poly
        .evaluate(&challenges)
        .map_err(|_| "could not evaluate initial poly")?;
    Ok(initial_poly_eval_at_challenge == claimed_sum)
}

/// Helper method for converting field elements to bytes
fn field_elements_to_bytes<F: PrimeField>(field_elements: &[F]) -> Vec<u8> {
    field_elements
        .iter()
        .map(|elem| elem.into_bigint().to_bytes_be())
        .collect::<Vec<Vec<u8>>>()
        .concat()
}

#[cfg(test)]
mod tests {
    use crate::polynomial::{poly, skip_one_and_sum_over_boolean_hypercube};
    use crate::sumcheck::{prove, verify};
    use ark_bn254::Fr;

    #[test]
    fn test_sumcheck() {
        // p(a, b, c) = 2ab + 3bc
        let poly = poly(
            3,
            vec![
                (Fr::from(2), vec![true, true, false]),
                (Fr::from(3), vec![false, true, true]),
            ],
        )
        .unwrap();

        let proof = prove(&poly, Fr::from(10));
        let verification_result = verify(&poly, &proof).unwrap();

        assert!(verification_result);
    }
}
