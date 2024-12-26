use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, Decimal, StdError};

use crate::core::pricing::PriceScaling;
use crate::defaults::constants::{
    DEFAULT_ADMIN_ADDRESS, DEFAULT_ROYALTY_ADDRESS, DEFAULT_ROYALTY_PERCENT,
};

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
            tile_admin_address: Addr::unchecked(DEFAULT_ADMIN_ADDRESS),
            tile_royalty_payment_address: DEFAULT_ROYALTY_ADDRESS.to_string(),
            tile_royalty_fee_percent: Decimal::percent(DEFAULT_ROYALTY_PERCENT),
            price_scaling: Default::default(),
        }
    }
}

impl Config {
    pub fn validate(&self, api: &dyn Api) -> Result<(), StdError> {
        // Validate price scaling
        self.price_scaling.validate()?;

        // Validate royalty fee
        if self.tile_royalty_fee_percent > Decimal::percent(100) {
            return Err(StdError::generic_err("Royalty fee cannot exceed 100%"));
        }

        // Validate royalty payment address
        if self.tile_royalty_payment_address.is_empty() {
            return Err(StdError::generic_err(
                "Royalty payment address cannot be empty",
            ));
        }
        api.addr_validate(&self.tile_royalty_payment_address)?;

        Ok(())
    }
}
