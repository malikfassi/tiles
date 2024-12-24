// Protocol constants that should never change
pub const PIXELS_PER_TILE: u32 = 100;
pub const TILE_SIZE: u32 = 10; // 10x10 grid
pub const MIN_UPDATE_INTERVAL: u64 = 60; // 1 minute minimum between updates
pub const MAX_UPDATE_INTERVAL: u64 = 31_536_000; // 1 year maximum update window

// Color validation
pub const COLOR_FORMAT: &str = "^#[0-9A-Fa-f]{6}$";
pub const DEFAULT_COLOR: &str = "#FFFFFF"; // Default white color

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Network constants
pub const NATIVE_DENOM: &str = "ustars";

// Pixel constants
pub const PIXEL_MIN_EXPIRATION: u64 = 60; // 1 minute
pub const PIXEL_MAX_EXPIRATION: u64 = 31_536_000; // 1 year

// Time constants (in seconds)
pub const MIN_EXPIRATION: u64 = 60;          // 1 minute
pub const MAX_EXPIRATION: u64 = 31_536_000;  // 1 year