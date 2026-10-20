use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;

use crate::error::NormalizationError;
use crate::error::validate_non_empty;

/// Standardizes values to mean `0` and population standard deviation `1`
/// (scikit-learn `StandardScaler` convention). Constant input maps to zeros.
///
/// # Errors
///
/// [`NormalizationError::EmptyInput`] when `values` is empty.
///
/// # Example
///
/// ```
/// use rust_decimal::Decimal;
///
/// use normalization_rs::z_score;
///
/// let values = [Decimal::ONE, Decimal::from(2), Decimal::from(3)];
/// let z = z_score(&values).unwrap();
/// assert_eq!(z[1], Decimal::ZERO);
/// ```
pub fn z_score(values: &[Decimal]) -> Result<Vec<Decimal>, NormalizationError> {
    validate_non_empty(values)?;

    let count = Decimal::from(values.len());
    let mean = values.iter().sum::<Decimal>() / count;
    let variance = values
        .iter()
        .map(|&value| {
            let deviation = value - mean;
            deviation * deviation
        })
        .sum::<Decimal>()
        / count;

    if variance.is_zero() {
        return Ok(vec![Decimal::ZERO; values.len()]);
    }

    let std_dev =
        variance.sqrt().ok_or(NormalizationError::NegativeVariance)?;
    Ok(values.iter().map(|&value| (value - mean) / std_dev).collect())
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::test_util::assert_all_close;
    use crate::test_util::decimals;

    #[test]
    fn matches_hand_computed_values() {
        // 1, 3, 5: mean 3, standard deviation sqrt(8/3)
        let z = z_score(&decimals(&[1, 3, 5])).unwrap();
        assert_all_close(
            &z,
            &[
                Decimal::new(-1_224_744_871_391_589, 15),
                Decimal::ZERO,
                Decimal::new(1_224_744_871_391_589, 15),
            ],
        );
    }

    #[test]
    fn constant_values_map_to_zero() {
        let z = z_score(&decimals(&[7, 7, 7])).unwrap();
        assert_eq!(z, vec![Decimal::ZERO; 3]);
    }

    #[test]
    fn single_value_maps_to_zero() {
        let z = z_score(&decimals(&[42])).unwrap();
        assert_eq!(z, vec![Decimal::ZERO]);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert_eq!(z_score(&[]), Err(NormalizationError::EmptyInput));
    }

    #[test]
    fn produces_zero_mean_and_unit_variance() {
        let values: Vec<Decimal> =
            (0..64).map(|i| Decimal::from(i * 31 % 23 - 11)).collect();
        let z = z_score(&values).unwrap();

        let count = Decimal::from(values.len());
        let mean = z.iter().sum::<Decimal>() / count;
        let variance = z
            .iter()
            .map(|&value| (value - mean) * (value - mean))
            .sum::<Decimal>()
            / count;

        assert!(mean.abs() < Decimal::new(1, 12), "mean {mean} is not ~0");
        assert!(
            (variance - Decimal::ONE).abs() < Decimal::new(1, 12),
            "variance {variance} is not ~1"
        );
    }
}
