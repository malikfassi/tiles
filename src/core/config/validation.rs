use cosmwasm_std::Decimal;

use crate::contract::error::ContractError;
use crate::core::config::DecimalOps;
use super::PriceScaling;

pub fn validate_royalty_percent(percent: Decimal) -> Result<(), ContractError> {
    if percent > Decimal::percent(100) {
        return Err(ContractError::InvalidDevFeePercent {});
    }
    Ok(())
}

pub fn validate_price_scaling<D: Clone + PartialEq + Into<Decimal> + DecimalOps>(
    price_scaling: &PriceScaling<D>,
) -> Result<(), ContractError> {
    let hour_1: Decimal = price_scaling.hour_1_price.clone().into();
    let hour_12: Decimal = price_scaling.hour_12_price.clone().into();
    let hour_24: Decimal = price_scaling.hour_24_price.clone().into();
    let quadratic_base: Decimal = price_scaling.quadratic_base.clone().into();

    if hour_1.is_zero()
        || hour_12.is_zero()
        || hour_24.is_zero()
        || quadratic_base.is_zero()
    {
        return Err(ContractError::InvalidPriceScaling {});
    }

    Ok(())
} 