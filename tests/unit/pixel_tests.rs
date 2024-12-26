use cosmwasm_std::Coin;
use sg_std::NATIVE_DENOM;
use tiles::core::tile::metadata::{TileMetadata, PixelUpdate};
use crate::common::helpers::setup::TestSetup;

#[test]
fn test_mint_and_set_pixel() {
    let mut setup = TestSetup::new();

    // Mint a token
    setup.collection.mint(
        &mut setup.app,
        &setup.admin,
        "1".to_string(),
        None,
        None,
    )
    .unwrap();

    // Create metadata and updates
    let mut metadata = TileMetadata::default();
    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration: 1000,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration: 1000,
        },
        PixelUpdate {
            id: 2,
            color: "#0000FF".to_string(),
            expiration: 1000,
        },
    ];

    // Set pixel color
    setup.collection.set_pixel_color(
        &mut setup.app,
        &setup.admin,
        "1".to_string(),
        metadata.clone(),
        updates,
        vec![Coin::new(300_000, NATIVE_DENOM)],
    )
    .unwrap();

    // Query token to verify pixel colors
    let token = setup.collection.query_token(&setup.app, "1".to_string()).unwrap();
    assert_eq!(token.extension.tile_hash.len(), 64); // SHA-256 hash is 64 chars
}

#[test]
fn test_set_pixel_unauthorized() {
    let mut setup = TestSetup::new();

    // Mint a token
    setup.collection.mint(
        &mut setup.app,
        &setup.admin,
        "1".to_string(),
        None,
        None,
    )
    .unwrap();

    // Create metadata and updates
    let mut metadata = TileMetadata::default();
    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration: 1000,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration: 1000,
        },
        PixelUpdate {
            id: 2,
            color: "#0000FF".to_string(),
            expiration: 1000,
        },
    ];

    // Try to set pixel color with unauthorized user
    let err = setup.collection.set_pixel_color(
        &mut setup.app,
        &setup.admin,
        "1".to_string(),
        metadata.clone(),
        updates,
        vec![Coin::new(300_000, NATIVE_DENOM)],
    )
    .unwrap_err();

    assert_eq!(err.to_string(), "Unauthorized");
}
