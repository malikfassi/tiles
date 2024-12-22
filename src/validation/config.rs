use cosmwasm_std::{Decimal, Uint128};

use crate::error::ContractError;
use crate::msg::UpdateConfigMsg;
use crate::state::PriceScaling;

/// Basic config validation functions
pub mod basic {
    use super::*;

    /// Validates fee percentage is between 0 and 100
    pub fn fee_percent(percent: Decimal) -> Result<(), ContractError> {
        if percent > Decimal::percent(100) || percent < Decimal::zero() {
            return Err(ContractError::InvalidFeePercent {});
        }
        Ok(())
    }

    /// Validates price is non-zero
    pub fn price(amount: Uint128) -> Result<(), ContractError> {
        if amount.is_zero() {
            return Err(ContractError::InvalidPrice {});
        }
        Ok(())
    }
}

/// Composite config validation functions
pub mod composite {
    use super::*;

    /// Validates price scaling configuration
    pub fn price_scaling(scaling: &PriceScaling) -> Result<(), ContractError> {
        // Validate all prices are non-zero
        basic::price(scaling.hour_1_price)?;
        basic::price(scaling.hour_12_price)?;
        basic::price(scaling.hour_24_price)?;
        basic::price(scaling.quadratic_base)?;

        // Validate price tiers are properly ordered
        if scaling.hour_1_price > scaling.hour_12_price
            || scaling.hour_12_price > scaling.hour_24_price
        {
            return Err(ContractError::InvalidPriceScaling {});
        }

        Ok(())
    }
}

/// Validates all fields in a config update
pub fn validate_config_update(msg: &UpdateConfigMsg) -> Result<(), ContractError> {
    // Validate fee percent if provided
    if let Some(dev_fee_percent) = msg.dev_fee_percent {
        basic::fee_percent(dev_fee_percent)?;
    }

    // Validate base price if provided
    if let Some(base_price) = msg.base_price {
        basic::price(base_price)?;
    }

    // Validate price scaling if provided
    if let Some(ref price_scaling) = msg.price_scaling {
        composite::price_scaling(price_scaling)?;
    }

    Ok(())
} 