//! Unit tests for the lease rule (ADR 0004): who may paint when.
//!
//! This is the heart of the product, and it runs without any chain.

use cosmwasm_std::{Addr, Timestamp};
use tiles::contract::error::ContractError;
use tiles::core::tile::metadata::{PixelData, PixelUpdate, TileMetadata};
use tiles::core::validation::validate_updates_for_tile;
use tiles::defaults::constants::PIXEL_MIN_EXPIRATION;

fn update(id: u8, color: &str) -> PixelUpdate {
    PixelUpdate {
        id,
        color: color.to_string(),
        expiration_duration: PIXEL_MIN_EXPIRATION,
    }
}

/// A tile with pixel 3 under a lease held by `holder` until `expires_at`.
fn tile_with_lease(holder: &str, expires_at: u64) -> TileMetadata {
    let mut metadata = TileMetadata::default();
    metadata.pixels[3] = PixelData {
        id: 3,
        color: "#000000".to_string(),
        lease_expires_at: Timestamp::from_seconds(expires_at),
        leased_by: Some(Addr::unchecked(holder)),
        last_updated_at: Timestamp::from_seconds(1),
    };
    metadata
}

#[test]
fn anyone_can_write_a_free_pixel() {
    let metadata = TileMetadata::default();
    let result = validate_updates_for_tile(
        "1",
        &metadata,
        &[update(3, "#FFFFFF")],
        &Addr::unchecked("stranger"),
        Timestamp::from_seconds(1000),
    );
    assert!(result.is_ok(), "a never-painted pixel is open to anyone");
}

#[test]
fn anyone_can_write_an_expired_lease() {
    let metadata = tile_with_lease("someone", 1999);
    let result = validate_updates_for_tile(
        "1",
        &metadata,
        &[update(3, "#FFFFFF")],
        &Addr::unchecked("stranger"),
        Timestamp::from_seconds(2000),
    );
    assert!(result.is_ok(), "an expired lease frees the pixel");
}

#[test]
fn a_stranger_cannot_write_under_an_active_lease() {
    let metadata = tile_with_lease("holder", 2000);
    let result = validate_updates_for_tile(
        "1",
        &metadata,
        &[update(3, "#FFFFFF")],
        &Addr::unchecked("stranger"),
        Timestamp::from_seconds(1000),
    );
    assert!(
        matches!(result, Err(ContractError::PixelLeaseActive { .. })),
        "expected PixelLeaseActive, got {:?}",
        result
    );
}

#[test]
fn the_holder_can_extend_their_own_lease() {
    let metadata = tile_with_lease("holder", 2000);
    let result = validate_updates_for_tile(
        "1",
        &metadata,
        &[update(3, "#FFFFFF")],
        &Addr::unchecked("holder"),
        Timestamp::from_seconds(1000),
    );
    assert!(result.is_ok(), "the holder may refresh their own colour");
}

#[test]
fn a_lease_ending_exactly_now_is_free() {
    let metadata = tile_with_lease("holder", 2000);
    let result = validate_updates_for_tile(
        "1",
        &metadata,
        &[update(3, "#FFFFFF")],
        &Addr::unchecked("stranger"),
        Timestamp::from_seconds(2000),
    );
    assert!(
        result.is_ok(),
        "the lease ends at its timestamp, so the pixel is free"
    );
}

/// One protected pixel must not block a batch that targets other pixels.
#[test]
fn a_protected_pixel_blocks_the_whole_batch() {
    let metadata = tile_with_lease("holder", 2000);
    let result = validate_updates_for_tile(
        "1",
        &metadata,
        &[update(1, "#FFFFFF"), update(3, "#FFFFFF")],
        &Addr::unchecked("stranger"),
        Timestamp::from_seconds(1000),
    );
    assert!(
        result.is_err(),
        "a batch containing a protected pixel is refused as a whole"
    );
}
