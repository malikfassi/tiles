use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::Item;
use hex;
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sha2::{Digest, Sha256};

use crate::error::ContractError;
use crate::defaults::constants::DEFAULT_PIXEL_COLOR;

#[cw_serde]
pub struct Extension {
    pub tile_hash: String, // Hash of current off-chain metadata
}

#[cw_serde]
pub struct PixelData {
    pub id: u32,               // Position within tile (0 to pixels_per_tile-1)
    pub color: String,         // Hex color (#RRGGBB)
    pub expiration: u64,       // Timestamp when pixel expires
    pub last_updated_by: Addr, // Address that last updated the pixel
    pub last_updated_at: u64,  // Timestamp of last update
}

#[cw_serde]
pub struct TileMetadata {
    pub tile_id: String,
    pub pixels: Vec<PixelData>,
}

impl Extension {
    pub fn generate_hash(tile_id: &str, pixels: &[PixelData]) -> String {
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

    pub fn verify_metadata(
        &self,
        tile_id: &str,
        metadata: &TileMetadata,
    ) -> Result<(), ContractError> {
        let current_hash = Self::generate_hash(tile_id, &metadata.pixels);
        if current_hash != self.tile_hash {
            return Err(ContractError::HashMismatch {});
        }
        Ok(())
    }
}

impl PixelData {
    pub fn new_at_mint(id: u32, owner: Addr, creation_time: u64) -> Self {
        Self {
            id,
            color: DEFAULT_PIXEL_COLOR.to_string(),
            expiration: creation_time,
            last_updated_by: owner,
            last_updated_at: creation_time,
        }
    }
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,                                          // Contract admin
    pub minter: Addr,                                         // Minting contract address
    pub collection_info: CollectionInfo<RoyaltyInfoResponse>, // Collection info
    pub dev_address: Addr,                                    // Developer fee recipient
    pub dev_fee_percent: Decimal,                             // Fee on pixel updates (e.g., 5%)
    pub base_price: Uint128,                                  // Base price per pixel
    pub price_scaling: Option<PriceScaling>,                  // Price scaling parameters
}

#[cw_serde]
pub struct PriceScaling {
    pub hour_1_price: Uint128,   // ≤1 hour price
    pub hour_12_price: Uint128,  // ≤12 hours price
    pub hour_24_price: Uint128,  // ≤24 hours price
    pub quadratic_base: Uint128, // Base for >24 hours
}

pub const CONFIG: Item<Config> = Item::new("config");
