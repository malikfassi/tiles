use std::collections::HashSet;
use crate::contract::error::ContractError;
use crate::defaults::constants::{PIXELS_PER_TILE, PIXEL_MAX_EXPIRATION, PIXEL_MIN_EXPIRATION, DEFAULT_COLOR};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use sha2::{Digest, Sha256};

#[cw_serde]
pub struct TileMetadata {
    pub pixels: Vec<PixelData>,
}

impl Default for TileMetadata {
    fn default() -> Self {
        let pixels = (0..PIXELS_PER_TILE)
            .map(|id| PixelData {
                id,
                color: DEFAULT_COLOR.to_string(),
                expiration: 0, // Expired by default
                last_updated_by: Addr::unchecked(""), // Empty by default
                last_updated_at: 0,
            })
            .collect();

        Self { pixels }
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
            pixel.color = update.color;
            pixel.expiration = update.expiration;
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
pub struct PixelData {
    pub id: u32,               // 0-99
    pub color: String,         // #RRGGBB
    pub expiration: u64,       // Unix timestamp
    pub last_updated_by: Addr, // Last modifier
    pub last_updated_at: u64,  // Unix timestamp of last update
}

#[cw_serde]
pub struct PixelUpdate {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
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

        // Validate expiration is in the future and within bounds
        if self.expiration <= current_time {
            return Err(ContractError::InvalidConfig(
                "Expiration must be in the future".to_string(),
            ));
        }

        let duration = self.expiration.saturating_sub(current_time);
        if !(PIXEL_MIN_EXPIRATION..=PIXEL_MAX_EXPIRATION).contains(&duration) {
            return Err(ContractError::InvalidConfig(format!(
                "Expiration duration must be between {} and {} seconds",
                PIXEL_MIN_EXPIRATION, PIXEL_MAX_EXPIRATION
            )));
        }

        // Validate current pixel is expired or not set
        if current_pixel.expiration < current_time {
            return Err(ContractError::InvalidConfig(
                "Pixel is not expired yet".to_string(),
            ));
        }

        Ok(())
    }

    pub fn apply(&self, metadata: &mut TileMetadata, sender: &Addr, current_time: u64) {
        // Find existing pixel index
        let pixel_idx = metadata.pixels.iter().position(|p| p.id == self.id);

        match pixel_idx {
            Some(idx) => {
                // Update existing pixel
                let pixel = &mut metadata.pixels[idx];
                pixel.color = self.color.clone();
                pixel.expiration = self.expiration;
                pixel.last_updated_by = sender.clone();
                pixel.last_updated_at = current_time;
            }
            None => {
                // Create new pixel
                metadata.pixels.push(PixelData {
                    id: self.id,
                    color: self.color.clone(),
                    expiration: self.expiration,
                    last_updated_by: sender.clone(),
                    last_updated_at: current_time,
                });
            }
        }
    }
}
