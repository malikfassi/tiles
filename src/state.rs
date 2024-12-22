use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use sg721::{CollectionInfo, RoyaltyInfoResponse};

pub const CONFIG: Item<Config> = Item::new("config");
pub const TILE_STATES: Map<&str, TileState> = Map::new("tile_states");

pub const MAX_MESSAGE_SIZE: u32 = 128 * 1024; // 128KB

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub minter: Addr,
    pub collection_info: CollectionInfo<RoyaltyInfoResponse>,
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
pub struct TileState {
    pub tile_hash: String,
    pub pixels: Vec<PixelData>,
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
    pub last_updated_by: String,
}

impl Default for TileState {
    fn default() -> Self {
        Self {
            tile_hash: "".to_string(),
            pixels: vec![],
        }
    }
}

impl TileState {
    pub fn verify_metadata(&self, tile_id: &str, metadata: &TileMetadata) -> Result<(), crate::error::ContractError> {
        let current_hash = Self::generate_hash(tile_id, &metadata.pixels);
        if current_hash != self.tile_hash {
            return Err(crate::error::ContractError::HashMismatch {});
        }
        Ok(())
    }

    pub fn generate_hash(tile_id: &str, pixels: &[PixelData]) -> String {
        use sha2::{Sha256, Digest};
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
}