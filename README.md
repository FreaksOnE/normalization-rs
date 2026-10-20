# normalization-rs

Decimal-precision numeric normalization for financial data, built on [`rust_decimal`](https://crates.io/crates/rust_decimal).

## API

- `z_score` — standardize to mean 0, population standard deviation 1
- `min_max_scale` — rescale to the unit interval `[0, 1]`
- `log_returns` — continuous returns `ln(P_t) - ln(P_{t-1})` for strictly positive prices

All functions take `&[Decimal]` and return `Result<_, NormalizationError>`. Constant input maps to zeros.

## Example

```rust
use rust_decimal::Decimal;

use normalization_rs::{log_returns, min_max_scale, z_score};

let prices = [Decimal::from(100), Decimal::from(110), Decimal::from(121)];

let z = z_score(&prices)?;
let scaled = min_max_scale(&prices)?;
let returns = log_returns(&prices)?;
```

Run the demo with `cargo run --example demo`.

## License

MIT
