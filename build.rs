use serde_json::json;
use sg_std::NATIVE_DENOM;
use std::fs;
use std::path::Path;

// Import constants from the project
include!("src/defaults/constants.rs");

fn main() {
    // Create scripts/messages directory if it doesn't exist
    let messages_dir = Path::new("scripts/messages");
    fs::create_dir_all(messages_dir).unwrap();

    // Export constants to JSON using values from constants.rs
    let constants = json!({
        // Contract Info
        "CONTRACT_NAME": CONTRACT_NAME,

        // Chain Configuration
        "CHAIN_ID": CHAIN_ID,
        "NODE_URL": NODE_URL,
        "GAS_PRICE": GAS_PRICE,
        "GAS_ADJUSTMENT": GAS_ADJUSTMENT,
        "BROADCAST_MODE": BROADCAST_MODE,
        "BASE_TOKEN_URI": BASE_TOKEN_URI,
        "COLLECTION_URI": COLLECTION_URI,

        // Collection Configuration
        "COLLECTION_NAME": COLLECTION_NAME,
        "COLLECTION_SYMBOL": COLLECTION_SYMBOL,
        "COLLECTION_DESCRIPTION": COLLECTION_DESCRIPTION,

        // Token Configuration
        "TOKEN_DENOM": NATIVE_DENOM,
        "START_TIME": START_TIME,
        "DEFAULT_ROYALTY_SHARE": DEFAULT_ROYALTY_SHARE,
        "MAX_TOKEN_LIMIT": MAX_TOKEN_LIMIT,
        "MAX_PER_ADDRESS_LIMIT": MAX_PER_ADDRESS_LIMIT,
        "DEFAULT_COLOR": DEFAULT_COLOR,
        "TILE_SIZE": TILE_SIZE,
        "PIXELS_PER_TILE": PIXELS_PER_TILE,
        "PIXEL_MIN_EXPIRATION": PIXEL_MIN_EXPIRATION,
        "PIXEL_MAX_EXPIRATION": PIXEL_MAX_EXPIRATION,

        // Financial Configuration
        "MINT_PRICE": MINT_PRICE,
        "CREATION_FEE": CREATION_FEE,
        "MINT_FEE_BPS": MINT_FEE_BPS,
        "MAX_TRADING_OFFSET_SECS": MAX_TRADING_OFFSET_SECS,
        "MIN_MINT_PRICE": MIN_MINT_PRICE,
        "AIRDROP_MINT_PRICE": AIRDROP_MINT_PRICE,
        "AIRDROP_MINT_FEE_BPS": AIRDROP_MINT_FEE_BPS,
        "SHUFFLE_FEE": SHUFFLE_FEE,
    });

    // Write constants to JSON file
    let constants_file = messages_dir.join("constants.json");
    fs::write(
        constants_file,
        serde_json::to_string_pretty(&constants).unwrap(),
    )
    .unwrap();

    // Tell Cargo to rerun this script if constants.rs changes
    println!("cargo:rerun-if-changed=src/defaults/constants.rs");
}
