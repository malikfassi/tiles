use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PricingError {
    #[error("Invalid price scaling: {0}")]
    InvalidPriceScaling(String),
}

#[cw_serde]
#[derive(Default)]
pub struct PriceScaling {
    pub hour_1_price: Uint128,
    pub hour_12_price: Uint128,
    pub hour_24_price: Uint128,
    pub quadratic_base: Uint128,
}

impl PriceScaling {
    pub fn validate(&self) -> Result<(), PricingError> {
        if self.hour_1_price.is_zero() {
            return Err(PricingError::InvalidPriceScaling(
                "hour_1_price cannot be zero".to_string(),
            ));
        }
        if self.hour_12_price.is_zero() {
            return Err(PricingError::InvalidPriceScaling(
                "hour_12_price cannot be zero".to_string(),
            ));
        }
        if self.hour_24_price.is_zero() {
            return Err(PricingError::InvalidPriceScaling(
                "hour_24_price cannot be zero".to_string(),
            ));
        }
        if self.quadratic_base.is_zero() {
            return Err(PricingError::InvalidPriceScaling(
                "quadratic_base cannot be zero".to_string(),
            ));
        }
        Ok(())
    }

    pub fn calculate_total_price(&self, expirations: &[u64], current_time: u64) -> Uint128 {
        expirations
            .iter()
            .map(|expiration| self.calculate_price(*expiration, current_time))
            .sum()
    }

    pub fn calculate_price(&self, expiration: u64, current_time: u64) -> Uint128 {
        let duration = expiration.saturating_sub(current_time);

        // Step-based pricing for durations up to 24 hours
        if duration <= 3600 {
            // 1 hour
            self.hour_1_price
        } else if duration <= 43200 {
            // 12 hours
            self.hour_12_price
        } else if duration <= 86400 {
            // 24 hours
            self.hour_24_price
        } else {
            // Quadratic scaling based on seconds for durations beyond 24 hours
            let excess_seconds = duration - 86400;
            let quadratic_factor = excess_seconds.saturating_mul(excess_seconds);
            self.quadratic_base
                .saturating_add(Uint128::new(quadratic_factor.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_calculate_price() {
        let pricing = PriceScaling::default();
        let current_time = 1000;

        // Test 1 hour duration
        let price = pricing.calculate_price(current_time + 3600, current_time);
        assert_eq!(price, pricing.hour_1_price);

        // Test 12 hour duration
        let price = pricing.calculate_price(current_time + 43200, current_time);
        assert_eq!(price, pricing.hour_12_price);

        // Test 24 hour duration
        let price = pricing.calculate_price(current_time + 86400, current_time);
        assert_eq!(price, pricing.hour_24_price);

        // Test beyond 24 hours
        let price = pricing.calculate_price(current_time + 90000, current_time);
        assert!(price > pricing.quadratic_base);
    }

    #[test]
    fn test_calculate_total_price() {
        let pricing = PriceScaling::default();
        let current_time = 1000;
        let expirations = vec![
            current_time + 3600,  // 1 hour
            current_time + 43200, // 12 hours
            current_time + 86400, // 24 hours
        ];

        let total = pricing.calculate_total_price(&expirations, current_time);
        let expected = pricing.hour_1_price + pricing.hour_12_price + pricing.hour_24_price;
        assert_eq!(total, expected);
    }
}
