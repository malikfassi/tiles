use crate::defaults::constants::PIXELS_PER_TILE;
use crate::types::TileMetadata;
use crate::utils::pixel::create_default_pixel;

/// Create default metadata for a new tile
pub fn create_default_metadata(tile_id: &str) -> TileMetadata {
    let mut pixels = Vec::with_capacity(PIXELS_PER_TILE as usize);
    for i in 0..PIXELS_PER_TILE {
        pixels.push(create_default_pixel(i, None, 0));
    }
    TileMetadata {
        tile_id: tile_id.to_string(),
        pixels,
    }
} 