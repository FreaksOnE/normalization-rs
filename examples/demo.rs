use rust_decimal::Decimal;

use normalization_rs::NormalizationError;
use normalization_rs::log_returns;
use normalization_rs::min_max_scale;
use normalization_rs::z_score;

fn main() -> Result<(), NormalizationError> {
    let prices = [
        Decimal::from(100),
        Decimal::from(110),
        Decimal::from(121),
        Decimal::from(99),
    ];

    println!("prices:      {prices:?}");
    println!("z-score:     {:?}", z_score(&prices)?);
    println!("min-max:     {:?}", min_max_scale(&prices)?);
    println!("log returns: {:?}", log_returns(&prices)?);
    Ok(())
}
