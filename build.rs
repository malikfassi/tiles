use serde_json::json;
use std::fs;
use std::path::Path;

// Constants are parsed out of the source instead of included, so that the build script
// does not need the contract's dependencies (cosmwasm_std) at build time.
fn read_const(name: &str) -> String {
    let source = fs::read_to_string("src/defaults/constants.rs").expect("constants.rs is readable");
    for line in source.lines() {
        let line = line.trim();
        if !line.starts_with("pub const ") {
            continue;
        }
        let rest = &line["pub const ".len()..];
        let (key, value) = match rest.split_once(':') {
            Some(parts) => parts,
            None => continue,
        };
        if key.trim() != name {
            continue;
        }
        let value = value.trim_start_matches([' ', '&', 's', 't', 'r', 'u', '8', 'u', '1', '2']);
        let value = value.trim().trim_end_matches(';').trim();
        return value.trim_matches('"').to_string();
    }
    panic!("constant {} not found", name);
}

fn main() {
    let messages_dir = Path::new("scripts/messages");
    fs::create_dir_all(messages_dir).expect("scripts/messages directory is writable");

    let constants = json!({
        // Contract info
        "CONTRACT_NAME": read_const("CONTRACT_NAME"),

        // Chain configuration
        "CHAIN_ID": read_const("CHAIN_ID"),
        "NODE_URL": read_const("NODE_URL"),
        "GAS_PRICE": read_const("GAS_PRICE"),
        "GAS_ADJUSTMENT": read_const("GAS_ADJUSTMENT"),
        "BROADCAST_MODE": read_const("BROADCAST_MODE"),

        // Collection configuration
        "COLLECTION_NAME": read_const("COLLECTION_NAME"),
        "COLLECTION_SYMBOL": read_const("COLLECTION_SYMBOL"),
        "COLLECTION_DESCRIPTION": read_const("COLLECTION_DESCRIPTION"),
        "BASE_TOKEN_URI": read_const("BASE_TOKEN_URI"),
        "COLLECTION_URI": read_const("COLLECTION_URI"),

        // Token configuration
        "TOKEN_DENOM": read_const("NATIVE_DENOM"),
        "DEFAULT_COLOR": read_const("DEFAULT_COLOR"),
        "TILE_SIZE": read_const("TILE_SIZE"),
        "PIXELS_PER_TILE": read_const("PIXELS_PER_TILE"),
        "PIXEL_MIN_EXPIRATION": read_const("PIXEL_MIN_EXPIRATION"),
        "PIXEL_MAX_EXPIRATION": read_const("PIXEL_MAX_EXPIRATION"),

        // Financial configuration
        "MINT_PRICE": read_const("MINT_PRICE"),
        "MIN_PIXEL_PRICE": read_const("MIN_PIXEL_PRICE"),
        "COLLECTION_SHARE_PERCENT": read_const("COLLECTION_SHARE_PERCENT"),
        "PLATFORM_SHARE_PERCENT": read_const("PLATFORM_SHARE_PERCENT"),
    });

    let constants_file = messages_dir.join("constants.json");
    fs::write(
        constants_file,
        serde_json::to_string_pretty(&constants).expect("constants serialise"),
    )
    .expect("constants.json is writable");

    println!("cargo:rerun-if-changed=src/defaults/constants.rs");
}
