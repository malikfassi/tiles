use cosmwasm_std::{Uint128, Decimal};

use crate::contract::error::ContractError;
use super::PriceScaling;

pub fn calculate_pixel_price(
    price_scaling: &PriceScaling<Decimal>,
    expiration_hours: u64,
) -> Result<Uint128, ContractError> {
    let hours = expiration_hours;
    
    let price = match hours {
        0..=1 => price_scaling.hour_1_price,
        2..=12 => price_scaling.hour_12_price,
        13..=24 => price_scaling.hour_24_price,
        _ => {
            // Quadratic scaling for >24h
            let base = price_scaling.quadratic_base;
            let hours_decimal = Decimal::from_ratio(hours as u128, 1u128);
            let quadratic = hours_decimal * hours_decimal / Decimal::percent(1);
            base + quadratic
        }
    };

    Ok(Uint128::new((price * Uint128::new(100)).u128()))
}