use cosmwasm_std::testing::mock_dependencies;
use tiles::state::{TileState, TileMetadata};

#[test]
fn test_hash_generation() {
    let _deps = mock_dependencies();

    let tile_id = "1".to_string();
    let pixels = vec![];

    let hash = TileState::generate_hash(&tile_id, &pixels);
    assert!(!hash.is_empty());
}

#[test]
fn test_verify_metadata() {
    let _deps = mock_dependencies();

    let tile_id = "1".to_string();
    let pixels = vec![];

    let hash = TileState::generate_hash(&tile_id, &pixels);
    let state = TileState {
        tile_hash: hash,
        pixels: pixels.clone(),
    };

    let metadata = TileMetadata {
        tile_id: tile_id.clone(),
        pixels: pixels.clone(),
    };

    assert!(state.verify_metadata(&tile_id, &metadata).is_ok());
} 