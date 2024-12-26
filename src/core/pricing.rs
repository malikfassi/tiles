use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::defaults::constants::{
    DEFAULT_PRICE_1_HOUR,
    DEFAULT_PRICE_12_HOURS,
    DEFAULT_PRICE_24_HOURS,
    DEFAULT_PRICE_QUADRATIC_BASE,
    ONE_HOUR,
    TWELVE_HOURS,
    TWENTY_FOUR_HOURS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
    pub fn validate(&self) -> Result<(), StdError> {
        if self.hour_1_price.is_zero() 
            || self.hour_12_price.is_zero() 
            || self.hour_24_price.is_zero() 
            || self.quadratic_base.is_zero() {
            return Err(StdError::generic_err("Prices cannot be zero"));
        }

        if !(self.hour_1_price < self.hour_12_price 
            && self.hour_12_price < self.hour_24_price 
            && self.hour_24_price < self.quadratic_base) {
            return Err(StdError::generic_err("Prices must be strictly increasing: 1h < 12h < 24h < quadratic_base"));
        }

        Ok(())
    }

    pub fn calculate_price(&self, duration_seconds: u64) -> Uint128 {
        if duration_seconds <= ONE_HOUR {
            self.hour_1_price
        } else if duration_seconds <= TWELVE_HOURS {
            self.hour_12_price
        } else if duration_seconds <= TWENTY_FOUR_HOURS {
            self.hour_24_price
        } else {
            // Calculate quadratic price based on seconds beyond 24 hours
            let extra_seconds = duration_seconds.saturating_sub(TWENTY_FOUR_HOURS);
            self.quadratic_base + Uint128::from(extra_seconds * extra_seconds)
        }
    }

    pub fn calculate_total_price<'a>(&self, durations: impl Iterator<Item = &'a u64>) -> Uint128 {
        durations
            .map(|duration| self.calculate_price(*duration))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_price() {
        let pricing = PriceScaling::default();

        // Test 1 hour duration
        let price = pricing.calculate_price(ONE_HOUR);
        assert_eq!(price, pricing.hour_1_price);

        // Test 12 hour duration
        let price = pricing.calculate_price(TWELVE_HOURS);
        assert_eq!(price, pricing.hour_12_price);

        // Test 24 hour duration
        let price = pricing.calculate_price(TWENTY_FOUR_HOURS);
        assert_eq!(price, pricing.hour_24_price);

        // Test beyond 24 hours
        let extra_seconds = 1000;
        let price = pricing.calculate_price(TWENTY_FOUR_HOURS + extra_seconds);
        assert_eq!(
            price,
            pricing.quadratic_base + Uint128::from(extra_seconds * extra_seconds)
        );
    }

    #[test]
    fn test_calculate_total_price() {
        let pricing = PriceScaling::default();
        let durations = vec![
            ONE_HOUR,
            TWELVE_HOURS,
            TWENTY_FOUR_HOURS,
        ];

        let total = pricing.calculate_total_price(durations.iter());
        let expected = pricing.hour_1_price + pricing.hour_12_price + pricing.hour_24_price;
        assert_eq!(total, expected);
    }

    #[test]
    fn test_validate_price_scaling() {
        let valid = PriceScaling {
            hour_1_price: Uint128::new(100),
            hour_12_price: Uint128::new(200),
            hour_24_price: Uint128::new(300),
            quadratic_base: Uint128::new(400),
        };
        assert!(valid.validate().is_ok());

        let invalid = PriceScaling {
            hour_1_price: Uint128::zero(),
            hour_12_price: Uint128::new(200),
            hour_24_price: Uint128::new(300),
            quadratic_base: Uint128::new(400),
        };
        assert!(invalid.validate().is_err());
    }
}
