use cosmwasm_std::{Decimal, Uint128};

use crate::types::PriceScaling;

// =============================
// Minter Factory Configuration
// =============================
pub const DEFAULT_MINT_PRICE: Uint128 = Uint128::new(100_000_000); // 100 STARS
pub const DEFAULT_MAX_TOKENS: u32 = 10_000;
pub const DEFAULT_MAX_TOKENS_PER_MINT: u32 = 10;
pub const DEFAULT_MAX_PER_ADDRESS_LIMIT: u32 = 50;

// =============================
// SG721 Base Configuration
// =============================
pub const DEFAULT_COLLECTION_NAME: &str = "Tiles";
pub const DEFAULT_TOKEN_SYMBOL: &str = "TILE";
pub const DEFAULT_COLLECTION_DESCRIPTION: &str = "A collection of customizable pixel tiles";
pub const DEFAULT_COLLECTION_IMAGE: &str = "ipfs://bafkreihdxc7zcyxykx7xskopr4yfk5lsv4bih4j4euqj6xv3s4u6wqpjc4";
pub const DEFAULT_ROYALTY_PERCENT: Decimal = Decimal::percent(5);
pub const DEFAULT_ROYALTY_PAYMENT_ADDRESS: &str = "stars1..."; // TODO: Set this
pub const DEFAULT_START_TRADING_TIME: u64 = 0; // Immediate trading

// =============================
// Tiles Contract Configuration
// =============================
// Addresses
pub const DEFAULT_ADMIN: &str = "admin";
pub const DEFAULT_MINTER: &str = "minter";
pub const DEFAULT_TILES_ROYALTY_PAYMENT_ADDRESS: &str = "tiles_royalty";

// Royalty settings
pub const DEFAULT_TILES_ROYALTIES: Decimal = Decimal::percent(5);

// Pixel update pricing
pub const DEFAULT_PRICE_SCALING: PriceScaling = PriceScaling {
    hour_1_price: Uint128::new(100_000_000),  // 100 STARS
    hour_12_price: Uint128::new(200_000_000), // 200 STARS
    hour_24_price: Uint128::new(300_000_000), // 300 STARS
    quadratic_base: Uint128::new(400_000_000), // 400 STARS
};
