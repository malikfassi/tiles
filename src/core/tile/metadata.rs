use std::collections::HashSet;
use crate::contract::error::ContractError;
use crate::defaults::constants::{PIXELS_PER_TILE, PIXEL_MAX_EXPIRATION, PIXEL_MIN_EXPIRATION, DEFAULT_COLOR};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use sha2::{Digest, Sha256};

#[cw_serde]
pub struct PixelData {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
    pub last_updated_by: Addr,
    pub last_updated_at: u64,
}

impl Default for PixelData {
    fn default() -> Self {
        Self {
            id: 0,
            color: DEFAULT_COLOR.to_string(),
            expiration: 0,
            last_updated_by: Addr::unchecked(""),
            last_updated_at: 0,
        }
    }
}

#[cw_serde]
pub struct TileMetadata {
    pub pixels: Vec<PixelData>,
}

impl Default for TileMetadata {
    fn default() -> Self {
        Self {
            pixels: (0..PIXELS_PER_TILE).map(|_| PixelData::default()).collect(),
        }
    }
}

impl TileMetadata {
    pub fn apply_updates(
        &mut self,
        updates: Vec<PixelUpdate>,
        sender: &Addr,
        current_time: u64
    ) {
        // All updates are just modifications of existing pixels
        for update in updates {
            let pixel = &mut self.pixels[update.id as usize];
            pixel.color = update.color.clone();
            pixel.expiration = update.get_expiration_time(current_time);
            pixel.last_updated_by = sender.clone();
            pixel.last_updated_at = current_time;
        }
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        for pixel in &self.pixels {
            hasher.update(format!(
                "{}:{}:{}:{}:{}",
                pixel.id,
                pixel.color,
                pixel.expiration,
                pixel.last_updated_by,
                pixel.last_updated_at
            ));
        }
        format!("{:x}", hasher.finalize())
    }
}

#[cw_serde]
pub struct PixelUpdate {
    pub id: u32,
    pub color: String,
    pub duration_seconds: u64,  // Duration in seconds instead of expiration timestamp
}

impl PixelUpdate {
    pub fn validate(
        &self,
        current_pixel: &PixelData,
        current_time: u64,
    ) -> Result<(), ContractError> {
        // Validate pixel id
        if self.id >= PIXELS_PER_TILE {
            return Err(ContractError::InvalidConfig(format!(
                "Invalid pixel id: {}",
                self.id
            )));
        }

        // Validate color format (#RRGGBB)
        if !self.color.starts_with('#')
            || self.color.len() != 7
            || !self.color[1..].chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(ContractError::InvalidConfig(format!(
                "Invalid color format: {}",
                self.color
            )));
        }

        // Validate duration is within bounds
        if !(PIXEL_MIN_EXPIRATION..=PIXEL_MAX_EXPIRATION).contains(&self.duration_seconds) {
            return Err(ContractError::InvalidConfig(format!(
                "Duration must be between {} and {} seconds",
                PIXEL_MIN_EXPIRATION, PIXEL_MAX_EXPIRATION
            )));
        }

        // Validate current pixel is expired
        if current_pixel.expiration > current_time {
            return Err(ContractError::InvalidConfig(
                "Pixel is not expired yet".to_string(),
            ));
        }

        Ok(())
    }

    // Helper to get expiration timestamp from duration
    pub fn get_expiration_time(&self, current_time: u64) -> u64 {
        current_time.saturating_add(self.duration_seconds)
    }
}
