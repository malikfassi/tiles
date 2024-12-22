use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, StdResult};
use sg721_base::Sg721Contract;

use crate::defaults::constants::{
    MAX_EXPIRATION, MAX_PIXEL_UPDATES_PER_TILE, MAX_TILE_UPDATES_PER_MESSAGE,
    MIN_EXPIRATION, PIXELS_PER_TILE,
};
use crate::state::CONFIG;
use crate::types::{Config, Extension, PixelData};
use crate::utils::metadata;

/// Response for config query
#[cw_serde]
pub struct ConfigResponse {
    /// Current contract configuration
    pub config: Config,
    /// Protocol constants
    pub constants: ProtocolConstants,
}

/// Protocol constants included in config response
#[cw_serde]
pub struct ProtocolConstants {
    /// Maximum number of tiles that can be updated in one message
    pub max_tile_updates_per_message: u32,
    /// Maximum number of pixels that can be updated per tile
    pub max_pixel_updates_per_tile: u32,
    /// Total number of pixels per tile
    pub pixels_per_tile: u32,
    /// Minimum expiration time in seconds
    pub min_expiration: u64,
    /// Maximum expiration time in seconds
    pub max_expiration: u64,
}

/// Response for tile state query
#[cw_serde]
pub struct TileStateResponse {
    /// Token ID of the tile
    pub token_id: String,
    /// Owner of the tile
    pub owner: String,
    /// Current pixel states
    pub pixels: Vec<PixelData>,
}

/// Query the current contract configuration and protocol constants
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    
    Ok(ConfigResponse {
        config,
        constants: ProtocolConstants {
            max_tile_updates_per_message: MAX_TILE_UPDATES_PER_MESSAGE,
            max_pixel_updates_per_tile: MAX_PIXEL_UPDATES_PER_TILE,
            pixels_per_tile: PIXELS_PER_TILE,
            min_expiration: MIN_EXPIRATION,
            max_expiration: MAX_EXPIRATION,
        },
    })
}

/// Query the current state of a tile
pub fn query_tile_state(deps: Deps, token_id: String) -> StdResult<TileStateResponse> {
    let contract: Sg721Contract<Extension> = Sg721Contract::default();
    let token = contract.parent.tokens.load(deps.storage, &token_id)?;

    // Create default metadata if none exists
    let metadata = metadata::create_default_metadata(&token_id);

    Ok(TileStateResponse {
        token_id,
        owner: token.owner.to_string(),
        pixels: metadata.pixels,
    })
}
