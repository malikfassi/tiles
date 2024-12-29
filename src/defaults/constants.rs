// Protocol constants that should never change
pub const PIXELS_PER_TILE: u32 = 100;
pub const TILE_SIZE: u32 = 10; // 10x10 grid
pub const DEFAULT_COLOR: &str = "#FFFFFF"; // Default white color
pub const PIXEL_MIN_EXPIRATION: u64 = 3600; // 1 hour
pub const PIXEL_MAX_EXPIRATION: u64 = 86400; // 24 hours
pub const DEFAULT_ROYALTY_SHARE: u64 = 10; // 10% royalty share

// Time thresholds for pricing (in seconds)
pub const ONE_HOUR: u64 = 3600;
pub const TWELVE_HOURS: u64 = 43200;
pub const TWENTY_FOUR_HOURS: u64 = 86400;

// Conversion rate (do not modify)
pub const USTARS_PER_STARS: u128 = 1_000_000; // 1 STARS = 1,000,000 uSTARS

// Default price values in uSTARS (micro STARS)
pub const DEFAULT_PRICE_1_HOUR: u128 = 100_000; // 0.1 STARS
pub const DEFAULT_PRICE_12_HOURS: u128 = 200_000; // 0.2 STARS
pub const DEFAULT_PRICE_24_HOURS: u128 = 300_000; // 0.3 STARS
pub const DEFAULT_PRICE_QUADRATIC_BASE: u128 = 400_000; // 0.4 STARS

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Minting price values (in uSTARS)
pub const MINT_PRICE: u128 = 100_000_000; // 100 STARS
pub const CREATION_FEE: u128 = 1_000_000; // 1 STARS

// Vending minter constants
pub const MINT_FEE_BPS: u64 = 1000; // 10%
pub const MAX_TOKEN_LIMIT: u32 = 10000;
pub const MAX_PER_ADDRESS_LIMIT: u32 = 3;
pub const MAX_TRADING_OFFSET_SECS: u64 = 60 * 60 * 24 * 7; // 1 week
pub const MIN_MINT_PRICE: u128 = 0;
pub const AIRDROP_MINT_PRICE: u128 = 0;
pub const AIRDROP_MINT_FEE_BPS: u64 = 0;
pub const SHUFFLE_FEE: u128 = 0;

// Chain configuration
pub const CHAIN_ID: &str = "elgafar-1";
pub const NODE_URL: &str = "https://rpc.elgafar-1.stargaze-apis.com:443";
pub const GAS_PRICE: &str = "0.025";
pub const GAS_ADJUSTMENT: f64 = 1.3;
pub const BROADCAST_MODE: &str = "sync";

// Collection configuration
pub const COLLECTION_NAME: &str = "Tiles";
pub const COLLECTION_SYMBOL: &str = "TILE";
pub const COLLECTION_DESCRIPTION: &str = "A collaborative pixel art canvas on Stargaze";
pub const BASE_TOKEN_URI: &str =
    "ipfs://bafybeidrmkt5uzfpz66esvhk3qflp47reztskaijxlsfu4fysujxicw7mu";
pub const COLLECTION_URI: &str = "ipfs://QmXzzVdLPNZCG1RnCSxDCSu9TfFd7fpnnwt8GuXdjNkjZw";

// Start time configuration
pub const START_TIME: &str = "1625097600"; // Example timestamp, should be set appropriately

pub const DEPLOYER_ADDRESS: &str = "stars1pnet2e7tz7klwy48r7h3wl0n97td0haqjvs7mx";