use cosmwasm_std::{Decimal, Env};
use crate::error::ContractError;
use crate::defaults::constants::{PIXELS_PER_TILE, MIN_EXPIRATION, MAX_EXPIRATION, MAX_MESSAGE_SIZE};
use crate::msg::{SetPixelColorMsg, UpdateConfigMsg};

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
    if duration < MIN_EXPIRATION || duration > MAX_EXPIRATION {
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

/// Validates config update parameters
pub fn validate_config_update(msg: &UpdateConfigMsg) -> Result<(), ContractError> {
    if let Some(dev_fee_percent) = msg.dev_fee_percent {
        // Ensure fee percent is between 0 and 100
        if dev_fee_percent > Decimal::percent(100) || dev_fee_percent < Decimal::zero() {
            return Err(ContractError::InvalidFeePercent {});
        }
    }
    Ok(())
} 