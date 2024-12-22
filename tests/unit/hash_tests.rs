use cosmwasm_std::Addr;
use tiles::types::{Extension, PixelData, TileMetadata};
use tiles::utils::hash::{generate_tile_hash, verify_metadata};

#[test]
fn test_generate_tile_hash() {
    let pixels = vec![PixelData {
        id: 0,
        color: "#000000".to_string(),
        expiration: 0,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 0,
    }];

    let hash = generate_tile_hash("1", &pixels);
    assert!(!hash.is_empty());
}

#[test]
fn test_verify_metadata() {
    let pixels = vec![PixelData {
        id: 0,
        color: "#000000".to_string(),
        expiration: 0,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 0,
    }];

    let metadata = TileMetadata {
        tile_id: "1".to_string(),
        pixels: pixels.clone(),
    };

    let extension = Extension {
        tile_hash: generate_tile_hash("1", &pixels),
    };

    let result = verify_metadata(&extension, "1", &metadata);
    assert!(result.is_ok());
}

#[test]
fn test_verify_metadata_mismatch() {
    let pixels = vec![PixelData {
        id: 0,
        color: "#000000".to_string(),
        expiration: 0,
        last_updated_by: Addr::unchecked("owner"),
        last_updated_at: 0,
    }];

    let metadata = TileMetadata {
        tile_id: "1".to_string(),
        pixels: pixels.clone(),
    };

    let extension = Extension {
        tile_hash: "wrong_hash".to_string(),
    };

    let result = verify_metadata(&extension, "1", &metadata);
    assert!(result.is_err());
}
