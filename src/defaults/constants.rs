// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Tile dimensions (immutable)
pub const PIXELS_PER_TILE: u32 = 100; // 10x10 grid
pub const TILE_WIDTH: u32 = 10;
pub const TILE_HEIGHT: u32 = 10;

// Message size limits (protocol constraints)
pub const MAX_MESSAGE_SIZE: u32 = 128 * 1024; // 128KB
pub const MAX_PIXEL_UPDATES_PER_MESSAGE: u32 = 100;
pub const MAX_TILE_UPDATES_PER_MESSAGE: u32 = 10; // Maximum number of tiles that can be updated in one message
pub const MAX_PIXEL_UPDATES_PER_TILE: u32 = 25;   // Maximum number of pixels that can be updated per tile

// Time constraints (protocol rules)
pub const MIN_EXPIRATION: u64 = 60; // 1 minute
pub const MAX_EXPIRATION: u64 = 31_536_000; // 1 year

// Fee constraints (protocol rules)
pub const MAX_FEE_PERCENT: u64 = 100;
pub const MIN_FEE_PERCENT: u64 = 0;

// Chain specific
pub const NATIVE_DENOM: &str = "ustars";

// Protocol defaults (immutable)
pub const DEFAULT_PIXEL_COLOR: &str = "#FFFFFF"; // White
