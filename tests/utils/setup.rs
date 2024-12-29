use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use tiles::core::tile::metadata::PixelUpdate;

use crate::utils::{
    app::TestApp,
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    events::EventParser,
    launchpad::Launchpad,
    state::StateTracker,
    users::TestUsers,
};

pub struct TestSetup {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub minter: MinterContract,
    pub factory: FactoryContract,
    pub state: StateTracker,
}

impl TestSetup {
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

    pub fn mint_token(&mut self, buyer: &Addr) -> Result<u32> {
        let response = self.minter.mint(&mut self.app, buyer)?;
        let token_id = EventParser::extract_token_id(&response)?;
        self.state.track_mint(&response)?;
        Ok(token_id)
    }

    pub fn with_minted_token() -> Result<(Self, u32)> {
        let mut setup = Self::new()?;
        let buyer = setup.users.get_buyer().clone();
        let token_id = setup.mint_token(&buyer.address)?;
        Ok((setup, token_id))
    }

    pub fn update_pixel(
        &mut self,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
    ) -> Result<AppResponse> {
        let metadata = self.state.get_metadata(token_id)?;
        let response =
            self.tiles
                .update_pixel(&mut self.app, sender, token_id, updates.clone(), metadata)?;
        self.state
            .track_pixel_update(token_id, &updates, &response)?;
        Ok(response)
    }
}
