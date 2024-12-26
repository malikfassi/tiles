// Protocol constants that should never change
pub const PIXELS_PER_TILE: u32 = 100;
pub const TILE_SIZE: u32 = 10; // 10x10 grid
pub const DEFAULT_COLOR: &str = "#FFFFFF"; // Default white color
pub const PIXEL_MIN_EXPIRATION: u64 = 3600; // 1 hour
pub const PIXEL_MAX_EXPIRATION: u64 = 86400; // 24 hours

// Time thresholds for pricing (in seconds)
pub const ONE_HOUR: u64 = 3600;
pub const TWELVE_HOURS: u64 = 43200;
pub const TWENTY_FOUR_HOURS: u64 = 86400;

// Default config values
pub const DEFAULT_ADMIN_ADDRESS: &str = "DEFAULT_ADMIN_ADDRESS";
pub const DEFAULT_ROYALTY_ADDRESS: &str = "DEFAULT_ROYALTY_ADDRESS";
pub const DEFAULT_ROYALTY_PERCENT: u64 = 5;

// Default price values (in ustars)
pub const DEFAULT_PRICE_1_HOUR: u128 = 100_000_000;
pub const DEFAULT_PRICE_12_HOURS: u128 = 200_000_000;
pub const DEFAULT_PRICE_24_HOURS: u128 = 300_000_000;
pub const DEFAULT_PRICE_QUADRATIC_BASE: u128 = 400_000_000;

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Test constants
pub const MINT_PRICE: u128 = 100_000_000;
pub const CREATION_FEE: u128 = 1_000_000;
pub const TEST_CREATOR: &str = "creator";
pub const TEST_TOKEN_URI: &str = "ipfs://test/";

// Vending minter constants
pub const MINT_FEE_BPS: u64 = 1000;
pub const MAX_TOKEN_LIMIT: u32 = 10000;
pub const MAX_PER_ADDRESS_LIMIT: u32 = 3;
pub const MAX_TRADING_OFFSET_SECS: u64 = 60 * 60 * 24 * 7; // 1 week
pub const MIN_MINT_PRICE: u128 = 0;
pub const AIRDROP_MINT_PRICE: u128 = 0;
pub const AIRDROP_MINT_FEE_BPS: u64 = 0;
pub const SHUFFLE_FEE: u128 = 0;
