use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128, Empty};
use cw_storage_plus::Item;
use sg721::CollectionInfo;
use sha2::{Sha256, Digest};
use hex;



use crate::msg::TileUpdates;

pub const MAX_MESSAGE_SIZE: u32 = 128 * 1024;  // 128KB
pub const MIN_EXPIRATION: u64 = 60;          // 1 minute
pub const MAX_EXPIRATION: u64 = 31_536_000;  // 1 year
pub const PIXELS_PER_TILE: u32 = 100;       // 10x10 grid
pub const DEFAULT_PIXEL_COLOR: &str = "#FFFFFF";  // White

#[cw_serde]
pub struct Extension {
    pub tile_hash: String,
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

    pub fn verify_metadata(&self, tile_id: &str, metadata: &TileMetadata) -> Result<(), crate::error::ContractError> {
        let current_hash = Self::generate_hash(tile_id, &metadata.pixels);
        if current_hash != self.tile_hash {
            return Err(crate::error::ContractError::HashMismatch {});
        }
        Ok(())
    }

    pub fn apply_updates(
        &self,
        current_metadata: &TileMetadata,
        updates: &TileUpdates,
        sender: &Addr,
        current_time: u64,
    ) -> Result<String, crate::error::ContractError> {
        let mut new_metadata = current_metadata.clone();

        // Apply each pixel update
        for update in &updates.pixels {
            // Find and update pixel
            let pixel = new_metadata.pixels
                .iter_mut()
                .find(|p| p.id == update.id)
                .ok_or(crate::error::ContractError::InvalidPixelUpdate {})?;

            // Update pixel
            pixel.color = update.color.clone();
            pixel.expiration = update.expiration;
            pixel.last_updated_by = sender.clone();
            pixel.last_updated_at = current_time;
        }

        // Generate new hash
        Ok(Self::generate_hash(&current_metadata.tile_id, &new_metadata.pixels))
    }
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub minter: Addr,
    pub collection_info: CollectionInfo<Empty>,
    pub dev_address: Addr,
    pub dev_fee_percent: Decimal,
    pub base_price: Uint128,
    pub price_scaling: Option<PriceScaling>,
}

#[cw_serde]
pub struct PriceScaling {
    pub hour_1_price: Uint128,
    pub hour_12_price: Uint128,
    pub hour_24_price: Uint128,
    pub quadratic_base: Uint128,
}

#[cw_serde]
pub struct TileMetadata {
    pub tile_id: String,
    pub pixels: Vec<PixelData>,
}

#[cw_serde]
pub struct PixelData {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
    pub last_updated_by: Addr,
    pub last_updated_at: u64,
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

pub const CONFIG: Item<Config> = Item::new("config");