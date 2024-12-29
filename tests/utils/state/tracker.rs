use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use std::collections::HashMap;
use tiles::{
    core::{
        pricing::PriceScaling,
        tile::metadata::{PixelData, PixelUpdate, TileMetadata},
    },
    events::{
        EventData, EventType, InstantiatePriceScalingEventData, MintMetadataEventData,
        PixelUpdateEventData, PriceScalingUpdateEventData,
    },
};

use crate::utils::{assertions::ResponseAssertions, events::EventParser};

pub struct StateTracker {
    pub tiles: HashMap<u32, TileMetadata>,
    pub price_scaling: Option<PriceScaling>,
}

impl StateTracker {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            price_scaling: Some(PriceScaling::default()),
        }
    }

    pub fn get_metadata(&self, token_id: u32) -> Result<TileMetadata> {
        self.tiles
            .get(&token_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No metadata found for token {}", token_id))
    }

    pub fn get_price_scaling(&self) -> Result<PriceScaling> {
        self.price_scaling
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No price scaling found"))
    }

    pub fn track_instantiate(&mut self, response: &AppResponse) -> Result<()> {
        // Parse the instantiate price scaling event
        let config = EventParser::find_and_parse::<InstantiatePriceScalingEventData>(response)?;

        // Parse price scaling from JSON string
        let price_scaling: PriceScaling = serde_json::from_str(&config.price_scaling)
            .map_err(|e| anyhow::anyhow!("Failed to parse price scaling JSON: {}", e))?;

        self.price_scaling = Some(price_scaling);
        Ok(())
    }

    pub fn track_price_scaling_update(&mut self, response: &AppResponse) -> Result<()> {
        // Parse the price scaling update event
        let update = EventParser::find_and_parse::<PriceScalingUpdateEventData>(response)?;

        // Update price scaling
        self.price_scaling = Some(PriceScaling {
            hour_1_price: update.hour_1_price.into(),
            hour_12_price: update.hour_12_price.into(),
            hour_24_price: update.hour_24_price.into(),
            quadratic_base: update.quadratic_base.into(),
        });

        Ok(())
    }

    pub fn track_mint(&mut self, response: &AppResponse) -> Result<u32> {
        let token_id = EventParser::extract_token_id(response)?;

        // Parse the mint metadata event
        let metadata = EventParser::find_and_parse::<MintMetadataEventData>(response)?;

        // Create initial metadata with default pixels
        let mut tile_metadata = TileMetadata::default();

        // Update the pixels that were modified
        for pixel in metadata.new_pixels.iter() {
            tile_metadata.pixels[pixel.id as usize] = pixel.clone();
        }

        self.tiles.insert(token_id, tile_metadata);
        Ok(token_id)
    }

    pub fn track_pixel_update(
        &mut self,
        token_id: u32,
        _updates: &[PixelUpdate],
        response: &AppResponse,
    ) -> Result<()> {
        let metadata = self
            .tiles
            .get_mut(&token_id)
            .ok_or_else(|| anyhow::anyhow!("No metadata found for token {}", token_id))?;

        // Parse all pixel update events
        let events = EventParser::find_and_parse_many::<PixelUpdateEventData>(response)?;

        for parsed in events {
            // Update the pixels that were modified
            for pixel in parsed.new_pixels.iter() {
                metadata.pixels[pixel.id as usize] = pixel.clone();
            }
        }

        Ok(())
    }
}
