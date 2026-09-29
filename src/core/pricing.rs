use crate::defaults::constants::{
    DEFAULT_PRICE_12_HOURS, DEFAULT_PRICE_1_HOUR, DEFAULT_PRICE_24_HOURS,
    DEFAULT_PRICE_QUADRATIC_BASE,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use thiserror::Error;

const ONE_HOUR: u64 = 3600;
const TWELVE_HOURS: u64 = ONE_HOUR * 12;
const TWENTY_FOUR_HOURS: u64 = ONE_HOUR * 24;

#[derive(Error, Debug, PartialEq)]
pub enum PriceScalingError {
    #[error("Invalid price scaling: {0}")]
    InvalidPriceScaling(String),
}

#[cw_serde]
pub struct PriceScaling {
    pub hour_1_price: Uint128,
    pub hour_12_price: Uint128,
    pub hour_24_price: Uint128,
    pub quadratic_base: Uint128,
}

impl Default for PriceScaling {
    fn default() -> Self {
        Self {
            hour_1_price: Uint128::from(DEFAULT_PRICE_1_HOUR),
            hour_12_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
            hour_24_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
            quadratic_base: Uint128::from(DEFAULT_PRICE_QUADRATIC_BASE),
        }
    }
}

impl PriceScaling {
    pub fn validate(&self) -> Result<(), PriceScalingError> {
        if self.hour_1_price.is_zero()
            || self.hour_12_price.is_zero()
            || self.hour_24_price.is_zero()
            || self.quadratic_base.is_zero()
        {
            return Err(PriceScalingError::InvalidPriceScaling(
                "prices must be greater than zero".to_string(),
            ));
        }
        if self.hour_1_price > self.hour_12_price {
            return Err(PriceScalingError::InvalidPriceScaling(
                "hour_1_price must be less than or equal to hour_12_price".to_string(),
            ));
        }
        if self.hour_12_price > self.hour_24_price {
            return Err(PriceScalingError::InvalidPriceScaling(
                "hour_12_price must be less than or equal to hour_24_price".to_string(),
            ));
        }
        Ok(())
    }

    pub fn calculate_price(&self, duration_seconds: u64) -> Uint128 {
        if duration_seconds <= ONE_HOUR {
            self.hour_1_price
        } else if duration_seconds <= TWELVE_HOURS {
            interpolate(
                self.hour_1_price,
                self.hour_12_price,
                duration_seconds - ONE_HOUR,
                TWELVE_HOURS - ONE_HOUR,
            )
        } else if duration_seconds <= TWENTY_FOUR_HOURS {
            interpolate(
                self.hour_12_price,
                self.hour_24_price,
                duration_seconds - TWELVE_HOURS,
                TWENTY_FOUR_HOURS - TWELVE_HOURS,
            )
        } else {
            // Beyond 24 hours the price grows quadratically with the extra seconds.
            let extra_seconds = Uint128::from(duration_seconds - TWENTY_FOUR_HOURS);
            let extra = extra_seconds
                .checked_mul(extra_seconds)
                .unwrap_or(Uint128::MAX);
            self.quadratic_base.saturating_add(extra)
        }
    }

    pub fn calculate_total_price<'a>(&self, durations: impl Iterator<Item = &'a u64>) -> Uint128 {
        durations
            .map(|duration| self.calculate_price(*duration))
            .sum()
    }
}

/// Linear interpolation between two price points.
///
/// `elapsed` and `span` are in seconds; the result is computed with integer arithmetic
/// in parts per million, so no float and no `unwrap` is involved.
fn interpolate(from: Uint128, to: Uint128, elapsed: u64, span: u64) -> Uint128 {
    if span == 0 {
        return to;
    }

    const PRECISION: u128 = 1_000_000;

    let progress = Uint128::from(elapsed as u128)
        .checked_mul(Uint128::from(PRECISION))
        .unwrap_or(Uint128::MAX)
        .checked_div(Uint128::from(span as u128))
        .unwrap_or(Uint128::MAX);

    let delta = to.saturating_sub(from);
    let increment = delta
        .checked_mul(progress)
        .unwrap_or(Uint128::MAX)
        .checked_div(Uint128::from(PRECISION))
        .unwrap_or(Uint128::MAX);

    from.saturating_add(increment)
}
