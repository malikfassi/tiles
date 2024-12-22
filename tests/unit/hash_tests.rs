use cosmwasm_std::Addr;
use tiles::state::{Extension, PixelData, TileMetadata};

#[test]
fn test_hash_generation() {
    let tile_id = "1";
    let pixels = vec![
        PixelData {
            id: 0,
            color: "#FF0000".to_string(),
            expiration: 1000,
            last_updated_by: Addr::unchecked("owner"),
            last_updated_at: 500,
        },
        PixelData {
            id: 1,
            color: "#00FF00".to_string(),
            expiration: 2000,
            last_updated_by: Addr::unchecked("owner"),
            last_updated_at: 500,
        },
    ];

    let hash = Extension::generate_hash(tile_id, &pixels);
    assert!(!hash.is_empty());
}

#[test]
fn test_hash_verification() {
    let tile_id = "1";
    let pixels = vec![PixelData {
        id: 0,
        color: "#FF0000".to_string(),
        expiration: 1000,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 500,
    }];

    let hash = Extension::generate_hash(tile_id, &pixels);
    let extension = Extension { tile_hash: hash };

    let metadata = TileMetadata {
        tile_id: tile_id.to_string(),
        pixels: pixels.clone(),
    };

    assert!(extension.verify_metadata(tile_id, &metadata).is_ok());
}

#[test]
fn test_hash_mismatch() {
    let tile_id = "1";
    let pixels = vec![PixelData {
        id: 0,
        color: "#FF0000".to_string(),
        expiration: 1000,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 500,
    }];

    let hash = Extension::generate_hash(tile_id, &pixels);
    let extension = Extension { tile_hash: hash };

    let mut modified_pixels = pixels.clone();
    modified_pixels[0].color = "#00FF00".to_string();

    let metadata = TileMetadata {
        tile_id: tile_id.to_string(),
        pixels: modified_pixels,
    };

    assert!(extension.verify_metadata(tile_id, &metadata).is_err());
}
