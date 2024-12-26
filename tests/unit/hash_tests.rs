use cosmwasm_std::Addr;
use tiles::core::tile::{metadata::{PixelData, TileMetadata}, Tile};

#[test]
fn test_generate_hash() {
    let mut metadata = TileMetadata::default();
    metadata.pixels[0] = PixelData {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_timestamp: 3600,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 1000,
    };

    let tile = Tile {
        tile_hash: "".to_string(),
    };

    let hash = tile.generate_hash(&metadata);
    assert!(!hash.is_empty());
}

#[test]
fn test_verify_metadata_success() {
    // Create initial metadata
    let mut metadata = TileMetadata::default();
    metadata.pixels[0] = PixelData {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_timestamp: 3600,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 1000,
    };

    let tile = Tile {
        tile_hash: Tile::default().generate_hash(&metadata),
    };

    assert!(tile.verify_metadata(&metadata));
}

#[test]
fn test_verify_metadata_failure() {
    // Create initial metadata
    let mut metadata = TileMetadata::default();
    metadata.pixels[0] = PixelData {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_timestamp: 3600,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 1000,
    };

    // Create different metadata
    let mut different_metadata = TileMetadata::default();
    different_metadata.pixels[0] = PixelData {
        id: 0,
        color: "#00FF00".to_string(), // Different color
        expiration_timestamp: 3600,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 1000,
    };

    let tile = Tile {
        tile_hash: Tile::default().generate_hash(&metadata),
    };

    assert!(!tile.verify_metadata(&different_metadata));
}
