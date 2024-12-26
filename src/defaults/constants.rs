// Protocol constants that should never change
pub const PIXELS_PER_TILE: u32 = 100;
pub const TILE_SIZE: u32 = 10; // 10x10 grid

// Time constants (in seconds)
pub const PIXEL_MIN_EXPIRATION: u64 = 3600; // 1 hour
pub const PIXEL_MAX_EXPIRATION: u64 = 86400; // 24 hours

// Default config values
pub const DEFAULT_ADMIN_ADDRESS: &str = "DEFAULT_ADMIN_ADDRESS";
pub const DEFAULT_ROYALTY_ADDRESS: &str = "DEFAULT_ROYALTY_ADDRESS";
pub const DEFAULT_ROYALTY_PERCENT: u64 = 5;

// Default price values (in ustars) - can be changed after instantiation
pub const DEFAULT_PRICE_1_HOUR: u128 = 100_000_000;
pub const DEFAULT_PRICE_12_HOURS: u128 = 200_000_000;
pub const DEFAULT_PRICE_24_HOURS: u128 = 300_000_000;
pub const DEFAULT_PRICE_QUADRATIC_BASE: u128 = 400_000_000;

// Color validation
pub const DEFAULT_COLOR: &str = "#FFFFFF"; // Default white color

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Network constants
pub const NATIVE_DENOM: &str = "ustars";
