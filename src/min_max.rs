use rust_decimal::Decimal;

use crate::error::NormalizationError;
use crate::error::validate_non_empty;

/// Rescales values to the unit interval `[0, 1]`; constant input maps to zeros.
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
/// use normalization_rs::min_max_scale;
///
/// let values = [Decimal::ONE, Decimal::from(2), Decimal::from(3)];
/// let scaled = min_max_scale(&values).unwrap();
/// assert_eq!(scaled[1], Decimal::new(5, 1));
/// ```
pub fn min_max_scale(
    values: &[Decimal],
) -> Result<Vec<Decimal>, NormalizationError> {
    validate_non_empty(values)?;

    let (min, max) = values
        .iter()
        .copied()
        .fold((values[0], values[0]), |(min, max), value| {
            (min.min(value), max.max(value))
        });

    let span = max - min;
    if span.is_zero() {
        return Ok(vec![Decimal::ZERO; values.len()]);
    }

    Ok(values.iter().map(|&value| (value - min) / span).collect())
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::test_util::decimals;

    #[test]
    fn maps_values_to_the_unit_interval() {
        let scaled = min_max_scale(&decimals(&[2, 4, 6])).unwrap();
        assert_eq!(
            scaled,
            vec![Decimal::ZERO, Decimal::new(5, 1), Decimal::ONE]
        );
    }

    #[test]
    fn constant_values_map_to_zero() {
        let scaled = min_max_scale(&decimals(&[5, 5])).unwrap();
        assert_eq!(scaled, vec![Decimal::ZERO; 2]);
    }

    #[test]
    fn single_value_maps_to_zero() {
        let scaled = min_max_scale(&decimals(&[42])).unwrap();
        assert_eq!(scaled, vec![Decimal::ZERO]);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert_eq!(min_max_scale(&[]), Err(NormalizationError::EmptyInput));
    }
}
