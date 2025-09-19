pub fn classification_algorithm(
    values: Vec<Vec<f64>>,
    solutions: Vec<i8>,
    weights: Vec<f64>,
    bias: f64,
    eta: f64
) -> (Vec<f64>, f64)
{
    let mut final_weights = weights;
    let mut final_bias = bias;

    for (traits, solution) in values.into_iter().zip(solutions) {
        let sigma = (traits
            .iter()
            .zip(final_weights.iter())
            .fold(final_bias, |acc, (r#trait, weight)|
                acc + r#trait * weight
            ) >= 0f64
        ) as i8;

        for (i, r#trait) in traits.into_iter().enumerate() {
            let delta_bias = eta * (solution - sigma) as f64;
            let delta_weight = delta_bias * r#trait;

            final_bias += delta_bias;
            final_weights[i] += delta_weight;
        }
    }

    (final_weights, final_bias)
}