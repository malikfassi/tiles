//! Unit tests for message validation and the lease rule (ADR 0004).
//!
//! All of this runs without any chain: these are the contract's guarantees, in the
//! simplest form they can take.

use cosmwasm_std::{Addr, Timestamp};
use tiles::contract::error::ContractError;
use tiles::core::tile::metadata::{PixelData, PixelUpdate, TileMetadata};
use tiles::core::validation::{
    is_valid_hex_color, validate_update, validate_updates, validate_updates_for_tile,
};
use tiles::defaults::constants::{PIXELS_PER_TILE, PIXEL_MAX_EXPIRATION, PIXEL_MIN_EXPIRATION};

fn update(id: u8, color: &str, duration: u64) -> PixelUpdate {
    PixelUpdate {
        id,
        color: color.to_string(),
        expiration_duration: duration,
    }
}

fn held_pixel(id: u8, holder: &str, expires_at: u64) -> PixelData {
    PixelData {
        id,
        color: "#000000".to_string(),
        lease_expires_at: Timestamp::from_seconds(expires_at),
        leased_by: Some(Addr::unchecked(holder)),
        last_updated_at: Timestamp::from_seconds(1),
    }
}

// ---- Colour format ----

#[test]
fn accepts_a_well_formed_colour() {
    assert!(is_valid_hex_color("#FFFFFF"));
    assert!(is_valid_hex_color("#abcdef"));
    assert!(is_valid_hex_color("#123456"));
}

#[test]
fn rejects_malformed_colours() {
    for bad in ["#FFF", "#FFFFFFF", "FFFFFF", "#GGGGGG", "", "#12345g"] {
        assert!(!is_valid_hex_color(bad), "{} must be rejected", bad);
    }
}

// ---- Single update ----

#[test]
fn accepts_a_valid_update() {
    assert!(validate_update(&update(0, "#FFFFFF", PIXEL_MIN_EXPIRATION)).is_ok());
    assert!(validate_update(&update(99, "#000000", PIXEL_MAX_EXPIRATION)).is_ok());
}

#[test]
fn rejects_a_pixel_id_out_of_bounds() {
    let result = validate_update(&update(
        PIXELS_PER_TILE as u8,
        "#FFFFFF",
        PIXEL_MIN_EXPIRATION,
    ));
    assert!(
        matches!(result, Err(ContractError::InvalidPixelId { .. })),
        "expected InvalidPixelId, got {:?}",
        result
    );
}

#[test]
fn rejects_a_bad_colour_format() {
    let result = validate_update(&update(0, "rouge", PIXEL_MIN_EXPIRATION));
    assert!(
        matches!(result, Err(ContractError::InvalidColorFormat {})),
        "expected InvalidColorFormat, got {:?}",
        result
    );
}

#[test]
fn rejects_a_lease_shorter_than_the_minimum() {
    let result = validate_update(&update(0, "#FFFFFF", PIXEL_MIN_EXPIRATION - 1));
    assert!(
        matches!(result, Err(ContractError::InvalidLeaseTooShort { .. })),
        "expected InvalidLeaseTooShort, got {:?}",
        result
    );
}

#[test]
fn rejects_a_lease_longer_than_the_maximum() {
    let result = validate_update(&update(0, "#FFFFFF", PIXEL_MAX_EXPIRATION + 1));
    assert!(
        matches!(result, Err(ContractError::InvalidLeaseTooLong { .. })),
        "expected InvalidLeaseTooLong, got {:?}",
        result
    );
}

// ---- Update list ----

#[test]
fn rejects_an_empty_update_list() {
    let result = validate_updates(&[]);
    assert!(
        matches!(result, Err(ContractError::EmptyUpdates {})),
        "expected EmptyUpdates, got {:?}",
        result
    );
}

#[test]
fn rejects_the_same_pixel_twice() {
    let updates = vec![
        update(5, "#FFFFFF", PIXEL_MIN_EXPIRATION),
        update(5, "#000000", PIXEL_MIN_EXPIRATION),
    ];
    let result = validate_updates(&updates);
    assert!(
        matches!(result, Err(ContractError::DuplicatePixelId { id: 5 })),
        "expected DuplicatePixelId, got {:?}",
        result
    );
}

#[test]
fn accepts_distinct_pixels() {
    let updates = vec![
        update(1, "#FFFFFF", PIXEL_MIN_EXPIRATION),
        update(2, "#000000", PIXEL_MIN_EXPIRATION),
    ];
    assert!(validate_updates(&updates).is_ok());
}
