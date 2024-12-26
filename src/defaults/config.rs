use crate::core::pricing::PriceScaling;
use cosmwasm_std::{Decimal, Uint128};

// Default addresses
pub const DEFAULT_TILE_ADMIN_ADDRESS: &str = "creator";
pub const DEFAULT_TILE_ROYALTY_PAYMENT_ADDRESS: &str = "creator";

// Default royalty fee percent
pub const DEFAULT_TILE_ROYALTY_FEE_PERCENT: Decimal = Decimal::percent(5);

// Network configuration
pub const NATIVE_DENOM: &str = "ustars";

// Default price scaling
pub const DEFAULT_PRICE_SCALING: PriceScaling = PriceScaling {
    hour_1_price: Uint128::new(100_000_000),
    hour_12_price: Uint128::new(200_000_000),
    hour_24_price: Uint128::new(300_000_000),
    quadratic_base: Uint128::new(400_000_000),
};
