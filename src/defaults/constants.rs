// Protocol constants that should never change
pub const PIXELS_PER_TILE: u32 = 100;
pub const TILE_SIZE: u32 = 10; // 10x10 grid

// Time constants (in seconds)
pub const PIXEL_MIN_EXPIRATION: u64 = 3600;        // 1 hour
pub const PIXEL_MAX_EXPIRATION: u64 = 86400;       // 24 hours

// Color validation
pub const COLOR_FORMAT: &str = "^#[0-9A-Fa-f]{6}$";
pub const DEFAULT_COLOR: &str = "#FFFFFF"; // Default white color

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Network constants
pub const NATIVE_DENOM: &str = "ustars";