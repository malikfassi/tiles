use std::fmt::Debug;
use cosmwasm_std::{Addr, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{
    pricing::{PriceScaling, calculation::calculate_pixel_price},
    tile::metadata::TileMetadata,
};

pub mod validation;

pub trait AddressOps: Clone + Debug + PartialEq {
    fn as_str(&self) -> &str;
}

pub trait DecimalOps: Clone + Debug + PartialEq {
    fn is_zero(&self) -> bool;
}

impl AddressOps for Addr {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl DecimalOps for Decimal {
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config<A: Clone + Debug + PartialEq + AddressOps, D: Clone + Debug + PartialEq + DecimalOps> {
    pub admin: A,
    pub minter: A,
    pub dev_address: A,
    pub dev_fee_percent: D,
    pub base_price: u128,
    pub price_scaling: PriceScaling<D>,
}

impl<A: Clone + Debug + PartialEq + AddressOps> Config<A, Decimal> {
    pub fn calculate_fees(&self, updates: &TileMetadata) -> Result<(Uint128, Uint128), crate::contract::error::ContractError> {
        let mut total_amount = Uint128::zero();

        // Sum up price for each pixel update
        for pixel in &updates.pixels {
            let hours = pixel.expiration / 3600;
            total_amount += calculate_pixel_price(&self.price_scaling, hours)?;
        }

        // Calculate fee split
        let dev_fee = total_amount * self.dev_fee_percent;
        let owner_payment = total_amount - dev_fee;

        Ok((dev_fee, owner_payment))
    }
}
