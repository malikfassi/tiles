use cosmwasm_std::Uint128;

use crate::error::ContractError;
use crate::types::PriceScaling;

/// Calculate price based on expiration duration
pub fn calculate_price(scaling: &PriceScaling, duration_seconds: u64) -> Uint128 {
    let hours = duration_seconds / 3600;
    match hours {
        0..=1 => scaling.hour_1_price,
        2..=12 => scaling.hour_12_price,
        13..=24 => scaling.hour_24_price,
        _ => {
            // Quadratic scaling for >24h
            scaling.quadratic_base + 
                Uint128::from(duration_seconds).pow(2) / Uint128::from(1_000_000u128)
        }
    }
}

/// Validate price scaling configuration
pub fn validate_price_scaling(scaling: &PriceScaling) -> Result<(), ContractError> {
    // Validate prices are non-zero
    if scaling.hour_1_price.is_zero()
        || scaling.hour_12_price.is_zero()
        || scaling.hour_24_price.is_zero()
        || scaling.quadratic_base.is_zero()
    {
        return Err(ContractError::InvalidPrice {
            value: "zero".to_string(),
        });
    }

    // Validate price tiers are properly ordered
    if scaling.hour_1_price > scaling.hour_12_price {
        return Err(ContractError::InvalidPriceScaling {
            error: format!(
                "1 hour price ({}) must be less than or equal to 12 hour price ({})",
                scaling.hour_1_price, scaling.hour_12_price
            ),
        });
    }
    if scaling.hour_12_price > scaling.hour_24_price {
        return Err(ContractError::InvalidPriceScaling {
            error: format!(
                "12 hour price ({}) must be less than or equal to 24 hour price ({})",
                scaling.hour_12_price, scaling.hour_24_price
            ),
        });
    }

    Ok(())
} 