/// Main test setup orchestrator that provides high-level test operations and state management.
/// This is the primary interface that test cases will interact with.
use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use tiles::core::tile::metadata::PixelUpdate;

use crate::utils::{
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    core::{app::TestApp, launchpad::Launchpad},
    state::{EventParser, StateTracker},
    TestUsers,
};

/// Context for pixel update operations
#[derive(Debug)]
pub struct PixelUpdateContext<'a> {
    pub sender: &'a Addr,
    pub token_id: u32,
    pub updates: Vec<PixelUpdate>,
}

/// Main test setup structure that holds all components needed for testing
pub struct TestSetup {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub factory: FactoryContract,
    pub minter: MinterContract,
    pub state: StateTracker,
}

impl TestSetup {
    /// Creates a new test setup with all necessary components initialized.
    /// This includes:
    /// - Test application
    /// - Contract deployments
    /// - User setup
    /// - Initial state tracking
    pub fn new() -> Result<Self> {
        let (mut launchpad, response) = Launchpad::setup()?;
        let mut state = StateTracker::new();

        // Initialize user balances
        launchpad.users.init_balances(&mut launchpad.app);

        // Track instantiation event
        state.track_instantiate(&response)?;

        Ok(Self {
            app: launchpad.app,
            users: launchpad.users,
            tiles: launchpad.tiles,
            minter: launchpad.minter,
            factory: launchpad.factory,
            state,
        })
    }

    /// Mints a new token for the specified buyer.
    ///
    /// # Arguments
    /// * `buyer` - Address that will receive the minted token
    ///
    /// # Returns
    /// * `Result<u32>` - The ID of the minted token
    pub fn mint_token(&mut self, buyer: &Addr) -> Result<u32> {
        let response = self.minter.mint(&mut self.app, buyer)?;
        let token_id = EventParser::extract_token_id(&response)?;
        self.state.track_mint(&response)?;
        Ok(token_id)
    }

    /// Creates a new test setup with a token already minted.
    /// Useful for tests that require an existing token.
    ///
    /// # Returns
    /// * `Result<(Self, u32)>` - The setup and the minted token ID
    pub fn with_minted_token() -> Result<(Self, u32)> {
        let mut setup = Self::new()?;
        let buyer = setup.users.get_buyer().clone();
        let token_id = setup.mint_token(&buyer.address)?;
        Ok((setup, token_id))
    }

    /// Updates pixels on a token with the specified changes.
    ///
    /// # Arguments
    /// * `sender` - Address of the sender
    /// * `token_id` - ID of the token to update
    /// * `updates` - List of pixel updates
    ///
    /// # Returns
    /// * `Result<AppResponse>` - The response from the update operation
    pub fn update_pixel(
        &mut self,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
    ) -> Result<AppResponse> {
        let current_metadata = self.state.get_token_metadata(token_id)?;
        let response = self.tiles
            .update_pixel(&mut self.app, sender, token_id, updates.clone(), current_metadata)?;
        
        // Track the update in our state
        self.state.track_pixel_update(token_id, &updates, &response)?;
        
        Ok(response)
    }

    /// Gets the current state tracker
    pub fn state(&self) -> &StateTracker {
        &self.state
    }
}
