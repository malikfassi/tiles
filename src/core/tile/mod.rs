
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Debug;

pub mod metadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Tile {
    pub tile_hash: String,
}

impl Tile {
    pub fn generate_hash(&self, metadata: &metadata::TileMetadata) -> String {
        let mut hasher = Sha256::new();
        for pixel in &metadata.pixels {
            hasher.update(pixel.id.to_be_bytes());
            hasher.update(pixel.color.as_bytes());
            hasher.update(pixel.expiration_timestamp.to_be_bytes());
            hasher.update(pixel.last_updated_by.as_bytes());
            hasher.update(pixel.last_updated_at.to_be_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_metadata(&self, metadata: &metadata::TileMetadata) -> bool {
        self.tile_hash == self.generate_hash(metadata)
    }
}
