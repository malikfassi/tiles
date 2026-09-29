use cosmwasm_std::{Decimal, Uint128};

// Protocol constants that should never change
pub const PIXELS_PER_TILE: usize = 100;
pub const TILE_SIZE: u32 = 10; // 10x10 grid
pub const DEFAULT_COLOR: &str = "#FFFFFF"; // Default white color
pub const PIXEL_MIN_EXPIRATION: u64 = 3600; // 1 hour
pub const PIXEL_MAX_EXPIRATION: u64 = 86400; // 24 hours

// Native denomination of the target chain (Cosmos Hub: ATOM).
// Stargaze 2.0 runs on the Cosmos Hub, where gas, mint and pixel payments are in ATOM.
//
// ADR 0005: this denom is the payment denom, hard-coded on purpose. ATOM is the "primary"
// token of Stargaze 2.0 and its marketplace pays royalties and fees in the denom of the sale.
// Gas is a separate flow paid by the caller on top of the price. Multi-token payment on
// Stargaze 2.0 is a frontend orchestration (Skip swap), not a contract capability.
pub const NATIVE_DENOM: &str = "uatom";

// Payment split applied to every pixel sale, in percent of the total price.
// The remainder goes to the tile owner. Collection share is capped at 10 %
// by the CW721 collection extension (Stargaze 2.0 limit).
pub const COLLECTION_SHARE_PERCENT: Decimal = Decimal::percent(5);
pub const PLATFORM_SHARE_PERCENT: Decimal = Decimal::percent(2);

// Price floor for a pixel sale, in micro ATOM.
pub const MIN_PIXEL_PRICE: Uint128 = Uint128::new(100_000); // 0.1 ATOM

// Time thresholds for pricing (in seconds)
pub const ONE_HOUR: u64 = 3600;
pub const TWELVE_HOURS: u64 = 43200;
pub const TWENTY_FOUR_HOURS: u64 = 86400;

// Default price values in micro ATOM (uatom)
pub const DEFAULT_PRICE_1_HOUR: u128 = 100_000; // 0.1 ATOM
pub const DEFAULT_PRICE_12_HOURS: u128 = 200_000; // 0.2 ATOM
pub const DEFAULT_PRICE_24_HOURS: u128 = 300_000; // 0.3 ATOM
pub const DEFAULT_PRICE_QUADRATIC_BASE: u128 = 400_000; // 0.4 ATOM

// Contract info
pub const CONTRACT_NAME: &str = "crates.io:tiles";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Minting price for a tile, in micro ATOM. The mint itself is performed by the
// contract minter, not by this contract, so this value is informational for clients.
pub const MINT_PRICE: u128 = 1_000_000; // 1 ATOM

// Chain configuration, used by the deployment scripts (scripts/messages/constants.json).
// Cosmos Hub mainnet. Override these values for a testnet deployment.
pub const CHAIN_ID: &str = "cosmoshub-4";
pub const NODE_URL: &str = "https://cosmos-rpc.polkachu.com:443";
pub const GAS_PRICE: &str = "0.025";
pub const GAS_ADJUSTMENT: f64 = 1.3;
pub const BROADCAST_MODE: &str = "sync";

// Collection configuration
pub const COLLECTION_NAME: &str = "Tiles";
pub const COLLECTION_SYMBOL: &str = "TILE";
pub const COLLECTION_DESCRIPTION: &str = "A collaborative pixel art canvas";
pub const BASE_TOKEN_URI: &str = "";
pub const COLLECTION_URI: &str = "";

// Start time configuration
pub const START_TIME: &str = "0";

pub const DEPLOYER_ADDRESS: &str = "";
