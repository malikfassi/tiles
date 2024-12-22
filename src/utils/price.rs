use cosmwasm_std::{Addr, Decimal, Uint128};

use crate::error::ContractError;
use crate::types::{Config, PriceScaling, PixelUpdate};

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

/// Calculate fees for a batch of pixel updates
pub fn calculate_batch_fees(
    config: &Config,
    updates: &[PixelUpdate],
    current_time: u64,
) -> Result<UpdateFees, ContractError> {
    let mut total_amount = Uint128::zero();

    // Calculate base price for each pixel update
    for update in updates {
        let duration = update.expiration.saturating_sub(current_time);
        total_amount += calculate_price(&config.price_scaling, duration);
    }

    // Calculate fee splits
    let royalty_fee = total_amount * config.tiles_royalties;
    let remaining_after_royalty = total_amount - royalty_fee;

    Ok(UpdateFees {
        total_amount,
        royalty_fee,
        royalty_address: config.tiles_royalty_payment_address.clone(),
    })
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

/// Validate payment amount matches required fees
pub fn validate_payment(
    payment_amount: Uint128,
    required_amount: Uint128,
) -> Result<(), ContractError> {
    if payment_amount < required_amount {
        return Err(ContractError::InsufficientFunds {
            required: required_amount,
            received: payment_amount,
        });
    }
    Ok(())
}

/// Struct to hold fee calculation results
#[derive(Debug, Clone)]
pub struct UpdateFees {
    pub total_amount: Uint128,
    pub royalty_fee: Uint128,
    pub royalty_address: Addr,
} 