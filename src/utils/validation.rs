use cosmwasm_std::Env;

use crate::defaults::constants::{
    MAX_TILE_UPDATES_PER_MESSAGE, MAX_PIXEL_UPDATES_PER_TILE, PIXELS_PER_TILE,
};
use crate::error::ContractError;
use crate::msg::SetPixelColorMsg;
use crate::types::{PixelData, TileMetadata};
use crate::utils::pixel;

/// Validates batch size limits for tile and pixel updates
pub fn validate_message_size(msg: &SetPixelColorMsg) -> Result<(), ContractError> {
    // Check number of tile updates
    if msg.updates.len() > MAX_TILE_UPDATES_PER_MESSAGE as usize {
        return Err(ContractError::BatchSizeExceeded {
            kind: "tile updates".to_string(),
            max: MAX_TILE_UPDATES_PER_MESSAGE,
            got: msg.updates.len() as u32,
        });
    }

    // Check number of pixel updates per tile
    for update in &msg.updates {
        if update.updates.pixels.len() > MAX_PIXEL_UPDATES_PER_TILE as usize {
            return Err(ContractError::BatchSizeExceeded {
                kind: format!("pixel updates for tile {}", update.tile_id),
                max: MAX_PIXEL_UPDATES_PER_TILE,
                got: update.updates.pixels.len() as u32,
            });
        }
    }

    Ok(())
}

/// Validates all fields in a pixel update
pub fn validate_pixel_update(
    id: u32,
    color: &str,
    expiration: u64,
    env: &Env,
) -> Result<(), ContractError> {
    pixel::validate_id(id)?;
    pixel::validate_color(color)?;
    pixel::validate_expiration(expiration, env)?;
    Ok(())
}

/// Validates a complete tile metadata structure
pub fn validate_tile_metadata(metadata: &TileMetadata) -> Result<(), ContractError> {
    // Check that we have exactly PIXELS_PER_TILE pixels
    if metadata.pixels.len() != PIXELS_PER_TILE as usize {
        return Err(ContractError::InvalidPixelUpdate {
            id: metadata.pixels.len() as u32,
            max: PIXELS_PER_TILE,
        });
    }

    // Check that pixel IDs are sequential and unique
    let mut seen_ids = vec![false; PIXELS_PER_TILE as usize];
    for pixel in &metadata.pixels {
        pixel::validate_id(pixel.id)?;
        if seen_ids[pixel.id as usize] {
            return Err(ContractError::InvalidPixelUpdate {
                id: pixel.id,
                max: PIXELS_PER_TILE - 1,
            });
        }
        seen_ids[pixel.id as usize] = true;
    }

    // Validate each pixel's color
    for pixel in &metadata.pixels {
        pixel::validate_color(&pixel.color)?;
    }

    Ok(())
}

/// Validates a pixel data structure
pub fn validate_pixel_data(pixel: &PixelData, env: &Env) -> Result<(), ContractError> {
    pixel::validate_id(pixel.id)?;
    pixel::validate_color(&pixel.color)?;
    pixel::validate_expiration(pixel.expiration, env)?;
    Ok(())
} 