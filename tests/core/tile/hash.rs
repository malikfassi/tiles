use tiles::core::tile::{
    metadata::{Pixel, TileMetadata},
    Tile,
};

#[test]
fn test_empty_tile_hash() {
    let metadata = TileMetadata::default();
    let tile = Tile::new(metadata);
    
    // Empty tile should have a consistent hash
    let hash = tile.compute_hash();
    assert!(!hash.is_empty());
    
    // Same empty tile should produce same hash
    let tile2 = Tile::new(TileMetadata::default());
    assert_eq!(tile.compute_hash(), tile2.compute_hash());
}

#[test]
fn test_pixel_update_changes_hash() {
    let mut metadata = TileMetadata::default();
    let mut tile = Tile::new(metadata.clone());
    let original_hash = tile.compute_hash();
    
    // Update a pixel
    metadata.pixels[0] = Pixel {
        color: "#FF0000".to_string(),
        expiration: 3600,
    };
    tile = Tile::new(metadata);
    
    // Hash should change after update
    let new_hash = tile.compute_hash();
    assert_ne!(original_hash, new_hash);
}

#[test]
fn test_hash_determinism() {
    let mut metadata = TileMetadata::default();
    metadata.pixels[0] = Pixel {
        color: "#FF0000".to_string(),
        expiration: 3600,
    };
    metadata.pixels[99] = Pixel {
        color: "#00FF00".to_string(),
        expiration: 7200,
    };
    
    // Create two identical tiles
    let tile1 = Tile::new(metadata.clone());
    let tile2 = Tile::new(metadata);
    
    // Should produce identical hashes
    assert_eq!(tile1.compute_hash(), tile2.compute_hash());
}

#[test]
fn test_hash_order_independence() {
    let mut metadata1 = TileMetadata::default();
    let mut metadata2 = TileMetadata::default();
    
    // Set pixels in different order
    metadata1.pixels[0] = Pixel {
        color: "#FF0000".to_string(),
        expiration: 3600,
    };
    metadata1.pixels[1] = Pixel {
        color: "#00FF00".to_string(),
        expiration: 7200,
    };
    
    metadata2.pixels[1] = Pixel {
        color: "#00FF00".to_string(),
        expiration: 7200,
    };
    metadata2.pixels[0] = Pixel {
        color: "#FF0000".to_string(),
        expiration: 3600,
    };
    
    let tile1 = Tile::new(metadata1);
    let tile2 = Tile::new(metadata2);
    
    // Order of updates shouldn't affect final hash
    assert_eq!(tile1.compute_hash(), tile2.compute_hash());
} 