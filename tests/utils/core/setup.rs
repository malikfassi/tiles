/// High level test context: one deployed collection plus the tracked state.
use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};

use crate::utils::{
    contracts::tiles::TilesContract,
    core::{app::TestApp, launchpad::Launchpad},
    state::StateTracker,
    TestUsers,
};

/// Single token id used across tests.
pub const TOKEN_ID: &str = "1";

/// Context for pixel update operations.
#[derive(Debug)]
pub struct PixelUpdateContext<'a> {
    pub sender: &'a Addr,
    pub token_id: u32,
    pub updates: Vec<PixelUpdate>,
}

/// Main test setup structure that holds all components needed for testing.
pub struct TestSetup {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub state: StateTracker,
}

impl TestSetup {
    /// Deploys the collection. No token is minted yet.
    pub fn new() -> Result<Self> {
        let (launchpad, _) = Launchpad::setup()?;
        let mut state = StateTracker::new();
        state.set_price_scaling(launchpad.tiles.query_price_scaling(&launchpad.app)?);

        Ok(Self {
            app: launchpad.app,
            users: launchpad.users,
            tiles: launchpad.tiles,
            state,
        })
    }

    /// The address that may update the price grid: the collection payment address,
    /// which the fixture sets to the collection creator.
    pub fn collection_creator(&self) -> Addr {
        self.users.get_buyer().address.clone()
    }

    /// Mints a tile to `buyer` and returns its token id.
    pub fn mint_token(&mut self, buyer: &Addr) -> Result<u32> {
        let response = self.tiles.mint(&mut self.app, buyer, TOKEN_ID)?;
        self.state.track_mint(&response)?;
        Ok(TOKEN_ID.parse().expect("token id is numeric"))
    }

    /// Deploys the collection and mints one tile to the buyer.
    pub fn with_minted_token() -> Result<(Self, u32)> {
        let mut setup = Self::new()?;
        let buyer = setup.users.get_buyer().address.clone();
        let token_id = setup.mint_token(&buyer)?;
        Ok((setup, token_id))
    }

    /// Colours pixels, reading the current state first so the hash matches.
    pub fn update_pixel(
        &mut self,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
    ) -> Result<AppResponse> {
        self.refresh_state();
        let current_metadata = self.state.get_token_metadata(token_id)?;
        let response = self.tiles.update_pixel(
            &mut self.app,
            sender,
            token_id,
            updates.clone(),
            current_metadata,
        )?;
        self.state
            .track_pixel_update(token_id, &updates, &response)?;
        Ok(response)
    }

    /// Current tile pixels, read straight from the contract.
    pub fn tile_metadata(&self, token_id: u32) -> Result<TileMetadata> {
        self.tiles.query_tile_pixels(&self.app, token_id)
    }

    /// Moves the chain clock forward, then resynchronises the tracked state.
    pub fn advance_time(&mut self, seconds: u64) {
        self.app.advance_time(seconds);
        self.refresh_state();
    }

    /// Reloads the tracked tile state from the contract.
    pub fn refresh_state(&mut self) {
        let metadata = self.tiles.query_tile_pixels(&self.app, 1);
        if let Ok(metadata) = metadata {
            self.state.set_token_metadata(1, metadata);
        }
    }

    pub fn state(&self) -> &StateTracker {
        &self.state
    }
}
