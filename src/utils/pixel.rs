use cosmwasm_std::{Addr, Env};

use crate::defaults::constants::{DEFAULT_PIXEL_COLOR, MAX_EXPIRATION, MIN_EXPIRATION, PIXELS_PER_TILE};
use crate::error::ContractError;
use crate::types::PixelData;

/// Validate a color string format (#RRGGBB)
pub fn validate_color(color: &str) -> Result<(), ContractError> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ContractError::InvalidColorFormat {
            color: color.to_string(),
        });
    }
    if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ContractError::InvalidColorFormat {
            color: color.to_string(),
        });
    }
    Ok(())
}

/// Validate expiration time relative to current block time
pub fn validate_expiration(expiration: u64, env: &Env) -> Result<(), ContractError> {
    let duration = expiration.saturating_sub(env.block.time.seconds());
    if !(MIN_EXPIRATION..=MAX_EXPIRATION).contains(&duration) {
        return Err(ContractError::InvalidExpiration {
            min: MIN_EXPIRATION,
            max: MAX_EXPIRATION,
            value: duration,
        });
    }
    Ok(())
}

/// Validate pixel ID is within bounds
pub fn validate_id(id: u32) -> Result<(), ContractError> {
    if id >= PIXELS_PER_TILE {
        return Err(ContractError::InvalidPixelUpdate {
            id,
            max: PIXELS_PER_TILE - 1,
        });
    }
    Ok(())
}

/// Create a new pixel with default values
pub fn create_default_pixel(id: u32, owner: Option<&Addr>, current_time: u64) -> PixelData {
    PixelData {
        id,
        color: DEFAULT_PIXEL_COLOR.to_string(),
        expiration: current_time + MIN_EXPIRATION,
        last_updated_by: owner.cloned().unwrap_or_else(|| Addr::unchecked("")),
        last_updated_at: current_time,
    }
} 