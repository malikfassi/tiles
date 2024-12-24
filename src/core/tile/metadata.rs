use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::contract::error::ContractError;

#[cw_serde]
pub struct TileMetadata {
    pub pixels: Vec<PixelData>,
}

#[cw_serde]
pub struct PixelData {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
    pub last_updated_by: Addr,
}

impl TileMetadata {
    pub fn new() -> Self {
        Self { pixels: vec![] }
    }

    pub fn update_pixel(
        &mut self,
        position: u32,
        color: String,
        expiration: u64,
        sender: Addr,
    ) -> Result<(), ContractError> {
        // Find existing pixel or create new one
        let pixel = self.pixels.iter_mut().find(|p| p.id == position);

        match pixel {
            Some(pixel) => {
                // Update existing pixel
                pixel.color = color;
                pixel.expiration = expiration;
                pixel.last_updated_by = sender;
            }
            None => {
                // Create new pixel
                self.pixels.push(PixelData {
                    id: position,
                    color,
                    expiration,
                    last_updated_by: sender,
                });
            }
        }

        Ok(())
    }

    pub fn get_pixel(&self, position: u32) -> Result<PixelData, ContractError> {
        self.pixels
            .iter()
            .find(|p| p.id == position)
            .cloned()
            .ok_or(ContractError::InvalidPixelPosition {})
    }
} 