use cosmwasm_std::Env;

use crate::defaults::constants::{
    MAX_EXPIRATION, MAX_MESSAGE_SIZE, MIN_EXPIRATION, PIXELS_PER_TILE,
};
use crate::error::ContractError;
use crate::msg::SetPixelColorMsg;
use crate::state::{PixelData, TileMetadata};

/// Validates a color string format (#RRGGBB)
pub fn validate_color(color: &str) -> Result<(), ContractError> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ContractError::InvalidColorFormat {});
    }
    if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ContractError::InvalidColorFormat {});
    }
    Ok(())
}

/// Validates expiration time relative to current block time
pub fn validate_expiration(expiration: u64, env: &Env) -> Result<(), ContractError> {
    let duration = expiration.saturating_sub(env.block.time.seconds());
    if !(MIN_EXPIRATION..=MAX_EXPIRATION).contains(&duration) {
        return Err(ContractError::InvalidExpiration {});
    }
    Ok(())
}

/// Validates pixel ID is within bounds
pub fn validate_pixel_id(id: u32) -> Result<(), ContractError> {
    if id >= PIXELS_PER_TILE {
        return Err(ContractError::InvalidPixelUpdate {});
    }
    Ok(())
}

/// Validates message size
pub fn validate_message_size(msg: &SetPixelColorMsg) -> Result<(), ContractError> {
    if msg.max_message_size > MAX_MESSAGE_SIZE {
        return Err(ContractError::MessageTooLarge {});
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
    validate_pixel_id(id)?;
    validate_color(color)?;
    validate_expiration(expiration, env)?;
    Ok(())
}

/// Validates a complete tile metadata structure
pub fn validate_tile_metadata(metadata: &TileMetadata) -> Result<(), ContractError> {
    // Check that we have exactly PIXELS_PER_TILE pixels
    if metadata.pixels.len() != PIXELS_PER_TILE as usize {
        return Err(ContractError::InvalidPixelUpdate {});
    }

    // Check that pixel IDs are sequential and unique
    let mut seen_ids = vec![false; PIXELS_PER_TILE as usize];
    for pixel in &metadata.pixels {
        validate_pixel_id(pixel.id)?;
        if seen_ids[pixel.id as usize] {
            return Err(ContractError::InvalidPixelUpdate {});
        }
        seen_ids[pixel.id as usize] = true;
    }

    // Validate each pixel's color
    for pixel in &metadata.pixels {
        validate_color(&pixel.color)?;
    }

    Ok(())
}

/// Validates a pixel data structure
pub fn validate_pixel_data(pixel: &PixelData, env: &Env) -> Result<(), ContractError> {
    validate_pixel_id(pixel.id)?;
    validate_color(&pixel.color)?;
    validate_expiration(pixel.expiration, env)?;
    Ok(())
} 