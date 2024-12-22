use hex;
use sha2::{Digest, Sha256};

use crate::error::ContractError;
use crate::types::{Extension, PixelData, TileMetadata};

/// Generate hash from tile metadata
pub fn generate_tile_hash(tile_id: &str, pixels: &[PixelData]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tile_id.as_bytes());
    for pixel in pixels {
        hasher.update(pixel.id.to_be_bytes());
        hasher.update(pixel.color.as_bytes());
        hasher.update(pixel.expiration.to_be_bytes());
        hasher.update(pixel.last_updated_by.as_bytes());
        hasher.update(pixel.last_updated_at.to_be_bytes());
    }
    hex::encode(hasher.finalize())
}

/// Verify metadata matches stored hash
pub fn verify_metadata(
    extension: &Extension,
    tile_id: &str,
    metadata: &TileMetadata,
) -> Result<(), ContractError> {
    let current_hash = generate_tile_hash(tile_id, &metadata.pixels);
    if current_hash != extension.tile_hash {
        return Err(ContractError::HashMismatch {});
    }
    Ok(())
} 