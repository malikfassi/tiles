use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

use crate::core::pricing::PriceScaling;

#[cw_serde]
pub struct Config {
    pub tile_admin_address: Addr,
    pub tile_royalty_payment_address: String,
    pub tile_royalty_fee_percent: Decimal,
    pub price_scaling: PriceScaling,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tile_admin_address: Addr::unchecked(""), // Will be set to creator
            tile_royalty_payment_address: "".to_string(),
            tile_royalty_fee_percent: Decimal::percent(5),
            price_scaling: Default::default(),
        }
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
