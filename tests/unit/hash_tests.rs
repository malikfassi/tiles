use cosmwasm_std::Addr;
use tiles::core::tile::metadata::{PixelData, TileMetadata};
use tiles::core::tile::Tile;

#[test]
fn test_hash_generation() {
    let tile_id = "1";
    let owner = Addr::unchecked("owner");
    let now = 1000u64;

    let pixels = vec![PixelData {
        id: 0,
        color: "#FFFFFF".to_string(),
        expiration: 3600,
        last_updated_by: owner.clone(),
        last_updated_at: now,
    }];

    let metadata = TileMetadata { pixels };
    let hash = Tile::generate_hash(tile_id, &metadata.pixels);

    assert!(!hash.is_empty());
}

#[test]
fn test_hash_verification() {
    let tile_id = "1";
    let owner = Addr::unchecked("owner");
    let now = 1000u64;

    let pixels = vec![PixelData {
        id: 0,
        color: "#FFFFFF".to_string(),
        expiration: 3600,
        last_updated_by: owner.clone(),
        last_updated_at: now,
    }];

    let metadata = TileMetadata { pixels };
    let hash = Tile::generate_hash(tile_id, &metadata.pixels);

    let extension = Tile {
        tile_hash: hash.clone(),
    };

    let result = extension.verify_metadata(tile_id, &metadata);
    assert!(result.is_ok());
}

#[test]
fn test_hash_mismatch() {
    let tile_id = "1";
    let owner = Addr::unchecked("owner");
    let now = 1000u64;

    let pixels = vec![PixelData {
        id: 0,
        color: "#FFFFFF".to_string(),
        expiration: 3600,
        last_updated_by: owner.clone(),
        last_updated_at: now,
    }];

    let metadata = TileMetadata { pixels };
    let hash = Tile::generate_hash(tile_id, &metadata.pixels);

    let different_pixels = vec![PixelData {
        id: 0,
        color: "#000000".to_string(), // Different color
        expiration: 3600,
        last_updated_by: owner.clone(),
        last_updated_at: now,
    }];

    let extension = Tile {
        tile_hash: hash,
    };

    let different_metadata = TileMetadata { pixels: different_pixels };
    let result = extension.verify_metadata(tile_id, &different_metadata);
    assert!(result.is_err());
}
