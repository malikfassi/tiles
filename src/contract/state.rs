use crate::core::pricing::PriceScaling;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::Item;

/// Installed once at instantiation, then immutable.
///
/// Holds the payment split that the contract applies to every pixel sale:
/// what the tiles collection takes, what the platform takes, and the price floor.
/// The tile owner always receives whatever is left.
#[cw_serde]
pub struct Config {
    /// Address receiving the collection share (royalties).
    pub collection_payment_address: Addr,
    /// Share of every pixel sale going to the collection, in percent of the total price.
    pub collection_share_percent: Decimal,
    /// Address receiving the platform share.
    pub platform_payment_address: Addr,
    /// Share of every pixel sale going to the platform, in percent of the total price.
    pub platform_share_percent: Decimal,
    /// Minimum price for a pixel sale, applied on top of the duration-based price.
    pub minimum_price: Uint128,
}

pub const PRICE_SCALING: Item<PriceScaling> = Item::new("price_scaling");
pub const CONFIG: Item<Config> = Item::new("config");
