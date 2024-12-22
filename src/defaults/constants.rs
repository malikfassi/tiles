use cosmwasm_std::Uint128;

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Tile dimensions
pub const PIXELS_PER_TILE: u32 = 100;  // 10x10 grid
pub const TILE_WIDTH: u32 = 10;
pub const TILE_HEIGHT: u32 = 10;

// Message size limits
pub const MAX_MESSAGE_SIZE: u32 = 128 * 1024;  // 128KB
pub const MAX_PIXEL_UPDATES_PER_MESSAGE: u32 = 100;

// Time constraints
pub const MIN_EXPIRATION: u64 = 60;          // 1 minute
pub const MAX_EXPIRATION: u64 = 31_536_000;  // 1 year
pub const DEFAULT_EXPIRATION: u64 = 3600;    // 1 hour

// Token related
pub const NATIVE_DENOM: &str = "ustars";
pub const DEFAULT_PIXEL_COLOR: &str = "#FFFFFF";  // White

// Price points (in ustars)
pub const BASE_PRICE: Uint128 = Uint128::new(100_000);
pub const HOUR_1_PRICE: Uint128 = Uint128::new(100_000);
pub const HOUR_12_PRICE: Uint128 = Uint128::new(200_000);
pub const HOUR_24_PRICE: Uint128 = Uint128::new(300_000);
pub const QUADRATIC_BASE: Uint128 = Uint128::new(400_000);

// Fee related
pub const DEFAULT_DEV_FEE_PERCENT: u64 = 5;  // 5%