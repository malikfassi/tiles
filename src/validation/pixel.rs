use cosmwasm_std::Env;

use crate::defaults::constants::{
    MAX_EXPIRATION, MAX_MESSAGE_SIZE, MIN_EXPIRATION, PIXELS_PER_TILE,
};
use crate::error::ContractError;
use crate::msg::SetPixelColorMsg;
use crate::state::{PixelData, TileMetadata};

/// Basic validation functions for individual pixel properties
pub mod basic {
    use super::*;

    /// Validates a color string format (#RRGGBB)
    pub fn color(color: &str) -> Result<(), ContractError> {
        if !color.starts_with('#') || color.len() != 7 {
            return Err(ContractError::InvalidColorFormat {});
        }
        if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ContractError::InvalidColorFormat {});
        }
        Ok(())
    }

    /// Validates expiration time relative to current block time
    pub fn expiration(expiration: u64, env: &Env) -> Result<(), ContractError> {
        let duration = expiration.saturating_sub(env.block.time.seconds());
        if !(MIN_EXPIRATION..=MAX_EXPIRATION).contains(&duration) {
            return Err(ContractError::InvalidExpiration {});
        }
        Ok(())
    }

    /// Validates pixel ID is within bounds
    pub fn id(id: u32) -> Result<(), ContractError> {
        if id >= PIXELS_PER_TILE {
            return Err(ContractError::InvalidPixelUpdate {});
        }
        Ok(())
    }
}

/// Message validation functions
pub mod message {
    use super::*;

    /// Validates message size
    pub fn size(msg: &SetPixelColorMsg) -> Result<(), ContractError> {
        if msg.max_message_size > MAX_MESSAGE_SIZE {
            return Err(ContractError::MessageTooLarge {});
        }
        Ok(())
    }
}

/// Composite validation functions for complex structures
pub mod composite {
    use super::*;

    /// Validates all fields in a pixel update
    pub fn pixel_update(
        id: u32,
        color: &str,
        expiration: u64,
        env: &Env,
    ) -> Result<(), ContractError> {
        basic::id(id)?;
        basic::color(color)?;
        basic::expiration(expiration, env)?;
        Ok(())
    }

    /// Validates a complete tile metadata structure
    pub fn tile_metadata(metadata: &TileMetadata) -> Result<(), ContractError> {
        // Check that we have exactly PIXELS_PER_TILE pixels
        if metadata.pixels.len() != PIXELS_PER_TILE as usize {
            return Err(ContractError::InvalidPixelUpdate {});
        }

        // Check that pixel IDs are sequential and unique
        let mut seen_ids = vec![false; PIXELS_PER_TILE as usize];
        for pixel in &metadata.pixels {
            basic::id(pixel.id)?;
            if seen_ids[pixel.id as usize] {
                return Err(ContractError::InvalidPixelUpdate {});
            }
            seen_ids[pixel.id as usize] = true;
        }

        // Validate each pixel's color
        for pixel in &metadata.pixels {
            basic::color(&pixel.color)?;
        }

        Ok(())
    }

    /// Validates a pixel data structure
    pub fn pixel_data(pixel: &PixelData, env: &Env) -> Result<(), ContractError> {
        basic::id(pixel.id)?;
        basic::color(&pixel.color)?;
        basic::expiration(pixel.expiration, env)?;
        Ok(())
    }
}

// Re-export commonly used functions at the module level for convenience
pub use basic::{color, expiration, id};
pub use composite::{pixel_data, pixel_update, tile_metadata};
pub use message::size as validate_message_size;