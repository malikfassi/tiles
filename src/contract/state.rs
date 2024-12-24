use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub minter: Addr,
    pub dev_address: Addr,
    pub dev_fee_percent: u64,
    pub base_price: u128,
    pub price_scaling: PriceScaling,
}

#[cw_serde]
pub struct PriceScaling {
    pub hour_1_price: u128,
    pub hour_12_price: u128,
    pub hour_24_price: u128,
    pub quadratic_base: u128,
}

pub const CONFIG: Item<Config> = Item::new("config");
