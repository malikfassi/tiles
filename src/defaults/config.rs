use cosmwasm_std::{Addr, Decimal, Uint128};
use sg721::RoyaltyInfoResponse;

use super::constants::*;
use crate::state::{Config, PriceScaling};

pub fn default_price_scaling() -> PriceScaling {
    PriceScaling {
        hour_1_price: HOUR_1_PRICE,
        hour_12_price: HOUR_12_PRICE,
        hour_24_price: HOUR_24_PRICE,
        quadratic_base: QUADRATIC_BASE,
    }
}

pub fn default_royalty_info() -> Option<RoyaltyInfoResponse> {
    None
}

pub fn default_dev_fee_percent() -> Decimal {
    Decimal::percent(DEFAULT_DEV_FEE_PERCENT)
}

pub fn default_base_price() -> Uint128 {
    BASE_PRICE
}

// Used in tests to create a default config
pub fn mock_config(
    admin: &Addr,
    minter: &Addr,
    collection_info: sg721::CollectionInfo<RoyaltyInfoResponse>,
) -> Config {
    Config {
        admin: admin.clone(),
        minter: minter.clone(),
        collection_info,
        dev_address: admin.clone(), // Use admin as dev address
        dev_fee_percent: default_dev_fee_percent(),
        base_price: default_base_price(),
        price_scaling: Some(default_price_scaling()),
    }
}