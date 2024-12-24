use std::fmt::Debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;

pub mod metadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Tile {
    pub tile_hash: String,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_hash: String::new(),
        }
    }
}

impl Tile {
    pub fn generate_hash(tile_id: &str, pixels: &[metadata::PixelData]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(tile_id.as_bytes());
        for pixel in pixels {
            hasher.update(pixel.id.to_be_bytes());
            hasher.update(pixel.color.as_bytes());
            hasher.update(pixel.expiration.to_be_bytes());
            hasher.update(pixel.last_updated_by.as_bytes());
        }
        hex::encode(hasher.finalize())
    }

    pub fn verify_metadata(&self, tile_id: &str, metadata: &metadata::TileMetadata) -> Result<(), crate::contract::error::ContractError> {
        let current_hash = Self::generate_hash(tile_id, &metadata.pixels);
        if current_hash != self.tile_hash {
            return Err(crate::contract::error::ContractError::HashMismatch {});
        }
        Ok(())
    }
}

pub const PIXELS_PER_TILE: u32 = 100;
