use cosmwasm_std::Addr;

use super::constants::{DEFAULT_PIXEL_COLOR, PIXELS_PER_TILE};
use crate::state::{PixelData, TileMetadata};

pub fn default_pixel(id: u32, owner: &Addr, creation_time: u64) -> PixelData {
    PixelData {
        id,
        color: DEFAULT_PIXEL_COLOR.to_string(),
        expiration: creation_time,
        last_updated_by: owner.clone(),
        last_updated_at: creation_time,
    }
}

pub fn default_tile_pixels(owner: &Addr, creation_time: u64) -> Vec<PixelData> {
    (0..PIXELS_PER_TILE)
        .map(|id| default_pixel(id, owner, creation_time))
        .collect()
}

pub fn default_tile_metadata(token_id: String, owner: &Addr, creation_time: u64) -> TileMetadata {
    TileMetadata {
        tile_id: token_id,
        pixels: default_tile_pixels(owner, creation_time),
    }
}