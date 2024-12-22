use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

/// Token extension for storing tile state
#[cw_serde]
pub struct Extension {
    /// Hash of current off-chain metadata
    pub tile_hash: String,
}

/// Individual pixel data within a tile
#[cw_serde]
pub struct PixelData {
    /// Position within tile (0 to PIXELS_PER_TILE-1)
    pub id: u32,
    /// Hex color (#RRGGBB)
    pub color: String,
    /// Timestamp when pixel expires
    pub expiration: u64,
    /// Address that last updated the pixel
    pub last_updated_by: Addr,
    /// Timestamp of last update
    pub last_updated_at: u64,
}

/// Complete tile metadata
#[cw_serde]
pub struct TileMetadata {
    /// Token ID of the tile
    pub tile_id: String,
    /// Pixel data (exactly PIXELS_PER_TILE elements)
    pub pixels: Vec<PixelData>,
}

/// Contract configuration
#[cw_serde]
pub struct Config {
    /// Minting contract address
    pub minter: Addr,
    /// Developer fee recipient address
    pub dev_address: Addr,
    /// Fee percentage on pixel updates (e.g., 5%)
    pub dev_fee_percent: Decimal,
    /// Base price per pixel update
    pub base_price: Uint128,
    /// Price scaling parameters
    pub price_scaling: PriceScaling,
}

/// Price scaling configuration
#[cw_serde]
pub struct PriceScaling {
    /// Price for updates expiring within 1 hour
    pub hour_1_price: Uint128,
    /// Price for updates expiring within 12 hours
    pub hour_12_price: Uint128,
    /// Price for updates expiring within 24 hours
    pub hour_24_price: Uint128,
    /// Base price for updates expiring after 24 hours
    pub quadratic_base: Uint128,
} 