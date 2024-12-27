use tiles::core::tile::{metadata::TileMetadata, Tile};

#[test]
fn test_empty_tile_hash() {
    let metadata = TileMetadata::default();
    let tile = Tile {
        tile_hash: metadata.hash(),
    };

    // Empty tile should have a consistent hash
    assert!(!tile.tile_hash.is_empty());

    // Same empty tile should produce same hash
    let tile2 = Tile {
        tile_hash: TileMetadata::default().hash(),
    };
    assert_eq!(tile.tile_hash, tile2.tile_hash);
}

#[test]
fn test_pixel_update_changes_hash() {
    let mut metadata = TileMetadata::default();
    let tile = Tile {
        tile_hash: metadata.hash(),
    };
    let original_hash = tile.tile_hash;

    // Update a pixel
    metadata.pixels[0].color = "#FF0000".to_string();
    metadata.pixels[0].expiration_timestamp = 3600;
    let tile = Tile {
        tile_hash: metadata.hash(),
    };

    // Hash should change after update
    let new_hash = tile.tile_hash;
    assert_ne!(original_hash, new_hash);
}

#[test]
fn test_hash_determinism() {
    let mut metadata = TileMetadata::default();
    metadata.pixels[0].color = "#FF0000".to_string();
    metadata.pixels[0].expiration_timestamp = 3600;
    metadata.pixels[99].color = "#00FF00".to_string();
    metadata.pixels[99].expiration_timestamp = 7200;

    // Create two identical tiles
    let tile1 = Tile {
        tile_hash: metadata.hash(),
    };
    let tile2 = Tile {
        tile_hash: metadata.hash(),
    };

    // Should produce identical hashes
    assert_eq!(tile1.tile_hash, tile2.tile_hash);
}

#[test]
fn test_hash_order_independence() {
    let mut metadata1 = TileMetadata::default();
    let mut metadata2 = TileMetadata::default();

    // Set pixels in different order
    metadata1.pixels[0].color = "#FF0000".to_string();
    metadata1.pixels[0].expiration_timestamp = 3600;
    metadata1.pixels[1].color = "#00FF00".to_string();
    metadata1.pixels[1].expiration_timestamp = 7200;

    metadata2.pixels[1].color = "#00FF00".to_string();
    metadata2.pixels[1].expiration_timestamp = 7200;
    metadata2.pixels[0].color = "#FF0000".to_string();
    metadata2.pixels[0].expiration_timestamp = 3600;

    let tile1 = Tile {
        tile_hash: metadata1.hash(),
    };
    let tile2 = Tile {
        tile_hash: metadata2.hash(),
    };

    // Order of updates shouldn't affect final hash
    assert_eq!(tile1.tile_hash, tile2.tile_hash);
}
