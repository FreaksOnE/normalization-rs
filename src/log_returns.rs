use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;

use crate::error::NormalizationError;
use crate::error::validate_non_empty;

/// Continuous (log) returns `ln(P_t) - ln(P_{t-1})` for consecutive strictly
/// positive prices; the result is one element shorter than the input.
///
/// # Errors
///
/// [`NormalizationError::EmptyInput`] for empty input,
/// [`NormalizationError::TooFewPrices`] for a single price, and
/// [`NormalizationError::NonPositivePrice`] for a zero or negative price.
///
/// # Example
///
/// ```
/// use rust_decimal::Decimal;
///
/// use normalization_rs::log_returns;
///
/// let prices = [Decimal::ONE, Decimal::from(2)];
/// let returns = log_returns(&prices).unwrap();
/// assert_eq!(returns.len(), 1);
/// ```
pub fn log_returns(
    prices: &[Decimal],
) -> Result<Vec<Decimal>, NormalizationError> {
    validate_non_empty(prices)?;
    if prices.len() < 2 {
        return Err(NormalizationError::TooFewPrices { len: prices.len() });
    }

    let log_prices = prices
        .iter()
        .enumerate()
        .map(|(index, &price)| {
            price.checked_ln().ok_or(NormalizationError::NonPositivePrice {
                index,
                value: price,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(log_prices.windows(2).map(|pair| pair[1] - pair[0]).collect())
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::test_util::assert_all_close;
    use crate::test_util::decimals;

    #[test]
    fn matches_log_of_consecutive_price_ratios() {
        let returns = log_returns(&decimals(&[100, 110, 121])).unwrap();
        // ln(1.1) twice
        assert_all_close(&returns, &[Decimal::new(95_310_179_804_325, 15); 2]);
    }

    #[test]
    fn returns_compound_to_the_total_log_return() {
        let returns = log_returns(&decimals(&[100, 110, 121])).unwrap();
        let total = returns.into_iter().sum::<Decimal>();
        // ln(121 / 100) = ln(1.21)
        let expected = Decimal::new(190_620_359_608_650, 15);
        assert!(
            (total - expected).abs() < Decimal::new(1, 12),
            "total {total} != {expected}"
        );
    }

    #[test]
    fn rejects_non_positive_prices() {
        assert_eq!(
            log_returns(&decimals(&[100, 0])),
            Err(NormalizationError::NonPositivePrice {
                index: 1,
                value: Decimal::ZERO,
            })
        );
        assert_eq!(
            log_returns(&decimals(&[-1, 2])),
            Err(NormalizationError::NonPositivePrice {
                index: 0,
                value: Decimal::from(-1),
            })
        );
    }

    #[test]
    fn rejects_a_single_price() {
        assert_eq!(
            log_returns(&decimals(&[100])),
            Err(NormalizationError::TooFewPrices { len: 1 })
        );
    }

    #[test]
    fn empty_input_is_rejected() {
        assert_eq!(log_returns(&[]), Err(NormalizationError::EmptyInput));
    }
}
