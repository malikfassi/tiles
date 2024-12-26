use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

use crate::core::pricing::PriceScaling;

#[cw_serde]
pub struct Config {
    pub dev_address: Addr,
    pub tile_royalty_payment_address: String,
    pub tile_royalty_fee_percent: Decimal,
    pub price_scaling: PriceScaling,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dev_address: Addr::unchecked(""),
            tile_royalty_payment_address: "".to_string(),
            tile_royalty_fee_percent: Decimal::percent(5),
            price_scaling: PriceScaling::default(),
        }
    }
}
