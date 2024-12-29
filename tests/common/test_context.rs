use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use std::collections::HashMap;
use tiles::{
    core::tile::metadata::{PixelData, PixelUpdate, TileMetadata},
    events::{EventData, MintMetadataEventData, PixelUpdateEventData},
};

use crate::common::{
    app::TestApp,
    contracts::{FactoryContract, MinterContract, TilesContract},
    launchpad::Launchpad,
    users::TestUsers,
    EventAssertions,
};

pub struct TestContext {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub minter: MinterContract,
    pub factory: FactoryContract,
    token_metadata: HashMap<u32, TileMetadata>,
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TestContext {
    pub fn new() -> Self {
        let launchpad = Launchpad::setup().unwrap();
        Self {
            app: launchpad.app,
            users: launchpad.users,
            tiles: launchpad.tiles,
            factory: launchpad.factory,
            minter: launchpad.minter,
            token_metadata: HashMap::new(),
        }
    }

    pub fn with_minted_token(owner: &Addr) -> Result<(Self, u32, AppResponse)> {
        let mut ctx = Self::new();
        let response = ctx.mint_token(owner)?;
        let token_id = EventAssertions::extract_token_id(&response);
        ctx.token_metadata.insert(token_id, TileMetadata::default());
        Ok((ctx, token_id, response))
    }

    pub fn mint_token(&mut self, buyer: &Addr) -> Result<AppResponse> {
        let response = self.minter.mint(&mut self.app, buyer)?;
        let token_id = EventAssertions::extract_token_id(&response);

        // Parse the mint metadata event to get the initial metadata
        let events = EventAssertions::find_events(
            &response,
            MintMetadataEventData::event_type().as_wasm_str().as_str(),
        );
        let event = events
            .first()
            .ok_or_else(|| anyhow::anyhow!("No mint metadata event found"))?;
        let metadata_event = MintMetadataEventData::try_from_event(event)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse mint metadata event"))?;

        // Create initial metadata with default state
        let metadata = TileMetadata::default();
        assert_eq!(
            metadata.hash(),
            metadata_event.tile_hash,
            "Initial metadata hash mismatch"
        );
        self.token_metadata.insert(token_id, metadata);

        Ok(response)
    }

    pub fn assert_balance(&self, addr: &Addr, denom: &str, expected: u128) {
        let balance = self
            .app
            .get_balance(addr, denom)
            .expect("Failed to get balance");
        assert_eq!(balance, expected, "Balance mismatch");
    }

    pub fn get_token_metadata(&self, token_id: u32) -> Result<TileMetadata> {
        self.token_metadata
            .get(&token_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No metadata found for token {}", token_id))
    }

    pub fn update_pixel(
        &mut self,
        owner: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
    ) -> Result<AppResponse> {
        let mut current_metadata = self.get_token_metadata(token_id)?;
        let response = self.tiles.update_pixel(
            &mut self.app,
            owner,
            token_id,
            updates.clone(),
            current_metadata.clone(),
        )?;

        // Parse the pixel update events to get the new pixel data
        for event in response.events.iter() {
            if event.ty == PixelUpdateEventData::event_type().as_wasm_str() {
                if let Some(pixel_event) = PixelUpdateEventData::try_from_event(event) {
                    let pixel_id = pixel_event.pixel_id as usize;
                    current_metadata.pixels[pixel_id] = PixelData {
                        id: pixel_event.pixel_id,
                        color: pixel_event.color,
                        expiration_timestamp: pixel_event.expiration_timestamp,
                        last_updated_by: pixel_event.last_updated_by,
                        last_updated_at: pixel_event.last_updated_at,
                    };
                }
            }
        }

        // Update our local cache
        self.token_metadata.insert(token_id, current_metadata);

        Ok(response)
    }
}
