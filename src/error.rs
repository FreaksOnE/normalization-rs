use rust_decimal::Decimal;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NormalizationError {
    #[error("cannot normalize an empty slice")]
    EmptyInput,

    #[error(
        "non-positive price {value} at index {index}; \
         log returns require strictly positive prices"
    )]
    NonPositivePrice { index: usize, value: Decimal },

    #[error("log returns require at least 2 prices, got {len}")]
    TooFewPrices { len: usize },

    #[error("cannot standardize: the computed variance is negative")]
    NegativeVariance,
}

pub const fn validate_non_empty(
    values: &[Decimal],
) -> Result<(), NormalizationError> {
    if values.is_empty() {
        return Err(NormalizationError::EmptyInput);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_positive_price_message_names_index_and_rule() {
        let error = NormalizationError::NonPositivePrice {
            index: 1,
            value: Decimal::ZERO,
        };
        let message = error.to_string();
        assert!(message.contains("0 at index 1"), "unexpected: {message}");
        assert!(message.contains("strictly positive"), "unexpected: {message}");
    }
}
