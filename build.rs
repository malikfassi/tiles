use serde_json::json;
use std::fs;
use std::path::Path;

/// Reads a `pub const NAME: TYPE = VALUE;` from the constants source and returns `VALUE`.
///
/// The value is extracted by splitting on the first `=` after the type, rather than by
/// trimming characters off the front: the type name varies (`&str`, `u64`, `Uint128`,
/// `Decimal`, `f64`) and would otherwise eat into the value. For string literals the
/// surrounding quotes are removed, and any trailing `//` comment is dropped.
fn read_const(name: &str) -> String {
    let source = fs::read_to_string("src/defaults/constants.rs").expect("constants.rs is readable");
    for line in source.lines() {
        let line = line.trim();
        if !line.starts_with("pub const ") {
            continue;
        }
        let rest = &line["pub const ".len()..];
        let (key, after_key) = match rest.split_once(':') {
            Some(parts) => parts,
            None => continue,
        };
        if key.trim() != name {
            continue;
        }
        // `after_key` is "TYPE = VALUE;", so the first `=` separates type from value.
        let value = match after_key.split_once('=') {
            Some((_ty, value)) => value,
            None => continue,
        };
        let value = value.trim();

        // A string literal is read between its quotes first, so a `//` inside it (an RPC
        // URL) is never mistaken for a comment. The terminator and any trailing comment
        // are handled after the closing quote.
        if let Some(after_quote) = value.strip_prefix('"') {
            let inner = match after_quote.split_once('"') {
                Some((inner, _trailing)) => inner,
                None => after_quote,
            };
            return inner.to_string();
        }

        // A numeric literal may carry a trailing comment, dropped here. The comment is cut
        // before the `;` because the comment itself can contain semicolons.
        let value = value.split("//").next().unwrap_or("").trim();
        let value = value.trim_end_matches(';').trim();
        return value.to_string();
    }
    panic!("constant {} not found", name);
}

/// Reads a numeric constant as a plain integer, dropping `_` separators and suffixes.
///
/// `Uint128::new(100_000)` yields "100000": only the digits of the innermost literal are
/// kept, so the `128` of `Uint128` never leaks into the value. Used for the money values,
/// where the Rust wrapper is noise for a shell or a JSON message.
fn read_const_number(name: &str) -> String {
    let raw = read_const(name);
    let innermost = raw
        .rsplit_once('(')
        .and_then(|(_, rest)| rest.split_once(')'))
        .map(|(inner, _)| inner)
        .unwrap_or(raw.as_str());
    let digits: String = innermost.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        panic!("constant {} is not numeric: {}", name, raw);
    }
    digits
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
        "KEYRING_BACKEND": read_const("KEYRING_BACKEND"),

        // Collection configuration
        "COLLECTION_NAME": read_const("COLLECTION_NAME"),
        "COLLECTION_SYMBOL": read_const("COLLECTION_SYMBOL"),
        "COLLECTION_DESCRIPTION": read_const("COLLECTION_DESCRIPTION"),
        "COLLECTION_URI": read_const("COLLECTION_URI"),
        "ROYALTY_SHARE": read_const("ROYALTY_SHARE"),

        // Token configuration
        "NATIVE_DENOM": read_const("NATIVE_DENOM"),
        "DEFAULT_COLOR": read_const("DEFAULT_COLOR"),
        "TILE_SIZE": read_const_number("TILE_SIZE"),
        "PIXELS_PER_TILE": read_const_number("PIXELS_PER_TILE"),
        "PIXEL_MIN_EXPIRATION": read_const_number("PIXEL_MIN_EXPIRATION"),
        "PIXEL_MAX_EXPIRATION": read_const_number("PIXEL_MAX_EXPIRATION"),

        // Financial configuration
        "MINT_PRICE": read_const_number("MINT_PRICE"),
        "MIN_PIXEL_PRICE": read_const_number("MIN_PIXEL_PRICE"),
        "COLLECTION_SHARE_BPS": read_const_number("COLLECTION_SHARE_BPS"),
        "PLATFORM_SHARE_BPS": read_const_number("PLATFORM_SHARE_BPS"),
    });

    let constants_file = messages_dir.join("constants.json");
    fs::write(
        constants_file,
        serde_json::to_string_pretty(&constants).expect("constants serialise"),
    )
    .expect("constants.json is writable");

    println!("cargo:rerun-if-changed=src/defaults/constants.rs");
}
