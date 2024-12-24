use cosmwasm_schema::cw_serde;
use cosmwasm_std::to_json_binary;
use sha2::{Digest, Sha256};

use crate::{
    contract::error::ContractError,
    core::tile::metadata::{PixelData, TileMetadata},
};

#[cw_serde]
pub struct TileState {
    pub tile_hash: String,
}

impl TileState {
    pub fn new(tile_id: &str) -> Self {
        Self {
            tile_hash: Self::generate_hash(tile_id, &[]),
        }
    }

    pub fn generate_hash(tile_id: &str, pixels: &[PixelData]) -> String {
        // Create hash input
        let hash_input = to_json_binary(&(tile_id, pixels)).unwrap();

        // Generate hash
        let hash = Sha256::new()
            .chain_update(hash_input)
            .finalize();

        // Convert to hex string
        hex::encode(hash)
    }

    pub fn verify_metadata(&self, tile_id: &str, metadata: &TileMetadata) -> Result<(), ContractError> {
        // Generate hash from current metadata
        let current_hash = Self::generate_hash(tile_id, &metadata.pixels);

        // Compare with stored hash
        if current_hash != self.tile_hash {
            return Err(ContractError::HashMismatch {});
        }

        Ok(())
    }

    pub fn update_pixel(
        &mut self,
        tile_id: &str,
        metadata: &mut TileMetadata,
        position: u32,
        color: String,
        expiration: u64,
        sender: String,
    ) -> Result<(), ContractError> {
        // Update metadata
        metadata.update_pixel(position, color, expiration, sender.parse().unwrap())?;

        // Update hash
        self.tile_hash = Self::generate_hash(tile_id, &metadata.pixels);

        Ok(())
    }
} 