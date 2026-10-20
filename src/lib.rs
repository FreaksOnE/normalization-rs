mod error;
mod log_returns;
mod min_max;
mod z_score;

#[cfg(test)]
mod test_util;

pub use error::NormalizationError;
pub use log_returns::log_returns;
pub use min_max::min_max_scale;
pub use z_score::z_score;
