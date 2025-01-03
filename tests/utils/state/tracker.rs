use anyhow::Result;
use cw_multi_test::AppResponse;
use tiles::core::{
    pricing::PriceScaling,
    tile::metadata::{PixelUpdate, TileMetadata},
};

use super::events::EventParser;

pub struct StateTracker {
    price_scaling: Option<PriceScaling>,
    token_metadata: std::collections::HashMap<u32, TileMetadata>,
}

impl Default for StateTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl StateTracker {
    pub fn new() -> Self {
        Self {
            price_scaling: None,
            token_metadata: std::collections::HashMap::new(),
        }
    }

    pub fn get_price_scaling(&self) -> Result<PriceScaling> {
        self.price_scaling
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No price scaling found"))
    }

    pub fn get_token_metadata(&self, token_id: u32) -> Result<TileMetadata> {
        self.token_metadata
            .get(&token_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No metadata found for token {}", token_id))
    }

    pub fn track_instantiate(&mut self, response: &AppResponse) -> Result<()> {
        let event = EventParser::parse_instantiate_event(response)?;
        let price_scaling: PriceScaling = serde_json::from_str(&event.price_scaling)?;
        self.price_scaling = Some(price_scaling);
        Ok(())
    }

    pub fn track_mint(&mut self, response: &AppResponse) -> Result<()> {
        let event = EventParser::parse_mint_metadata(response)?;
        let token_id = event.token_id.parse::<u32>()?;
        self.token_metadata
            .insert(token_id, TileMetadata::default());
        Ok(())
    }

    pub fn track_pixel_update(
        &mut self,
        token_id: u32,
        _updates: &[PixelUpdate],
        response: &AppResponse,
    ) -> Result<()> {
        let event = EventParser::parse_pixel_update(response)?;
        let metadata = self.token_metadata.entry(token_id).or_default();
        println!("event: {:?}", event);
        // Update pixels from event data
        for pixel_data in event.new_pixels {
            println!("pixel_data: {:?}", pixel_data);
            metadata.pixels[pixel_data.id as usize] = pixel_data.clone();
        }
        Ok(())
    }
}
