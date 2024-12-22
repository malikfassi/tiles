use cosmwasm_std::{Decimal, Uint128};

use crate::types::PriceScaling;

// Default expiration time
pub const DEFAULT_EXPIRATION: u64 = 3600; // 1 hour

// Default collection info
pub const DEFAULT_COLLECTION_DESCRIPTION: &str = "A collection of customizable pixel tiles";
pub const DEFAULT_COLLECTION_IMAGE: &str = "ipfs://bafkreihdxc7zcyxykx7xskopr4yfk5lsv4bih4j4euqj6xv3s4u6wqpjc4";

// Default price points (in ustars)
pub const DEFAULT_BASE_PRICE: Uint128 = Uint128::new(100_000);
pub const DEFAULT_HOUR_1_PRICE: Uint128 = Uint128::new(100_000);
pub const DEFAULT_HOUR_12_PRICE: Uint128 = Uint128::new(200_000);
pub const DEFAULT_HOUR_24_PRICE: Uint128 = Uint128::new(300_000);
pub const DEFAULT_QUADRATIC_BASE: Uint128 = Uint128::new(400_000);

// Default fee settings
pub const DEFAULT_DEV_FEE_PERCENT: Decimal = Decimal::percent(5);

// Default test values
pub const DEFAULT_INITIAL_BALANCE: u128 = 1_000_000_000;
pub const DEFAULT_MINT_PRICE: u128 = 100_000_000;
pub const DEFAULT_CREATION_FEE: u128 = 1_000_000;
pub const DEFAULT_MIN_MINT_PRICE: u128 = 100_000;
pub const DEFAULT_SHUFFLE_FEE: u128 = 500_000;
pub const DEFAULT_MINT_FEE_BPS: u64 = 1000;
pub const DEFAULT_MAX_TOKEN_LIMIT: u32 = 10000;
pub const DEFAULT_MAX_PER_ADDRESS_LIMIT: u32 = 50;
pub const DEFAULT_MAX_TRADING_OFFSET_SECS: u64 = 60 * 60 * 24 * 7; // 1 week

// Default configuration objects
pub const DEFAULT_PRICE_SCALING: PriceScaling = PriceScaling {
    hour_1_price: Uint128::new(100_000),
    hour_12_price: Uint128::new(200_000),
    hour_24_price: Uint128::new(300_000),
    quadratic_base: Uint128::new(400_000),
};
