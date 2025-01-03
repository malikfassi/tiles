/// Contract deployment and initialization orchestrator.
/// Handles the setup of all contracts needed for testing, including:
/// - Factory contract
/// - Minter contract
/// - Tiles (SG721) contract
use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;

use crate::utils::{
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    core::app::TestApp,
    TestUsers,
};

/// Main launchpad structure that handles contract deployment and setup
pub struct Launchpad {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub factory: FactoryContract,
    pub minter: MinterContract,
}

impl Launchpad {
    /// Creates a new empty launchpad instance with default configuration
    pub fn new_empty() -> Self {
        let mut app = TestApp::new();
        let users = TestUsers::new();
        users.init_balances(&mut app); // Initialize user balances
        let factory = FactoryContract::new(&mut app, "factory");

        Self {
            app,
            users,
            // Initialize with dummy addresses that will be updated later
            tiles: TilesContract::new(Addr::unchecked("tiles")),
            factory,
            minter: MinterContract::new(Addr::unchecked("minter")),
        }
    }

    /// Stores all contract code in the test environment
    ///
    /// # Returns
    /// * `Result<(u64, u64, u64)>` - Code IDs for (factory, minter, collection)
    pub fn store_contracts(&mut self) -> Result<(u64, u64, u64)> {
        // Store contracts in the correct order to get expected code IDs:
        // factory = 1, sg721 = 2, minter = 3
        let factory_code_id = FactoryContract::store_code(&mut self.app)?;
        let collection_code_id = TilesContract::store_code(&mut self.app)?;
        let minter_code_id = MinterContract::store_code(&mut self.app)?;

        Ok((factory_code_id, minter_code_id, collection_code_id))
    }

    /// Sets up the factory contract with the provided code IDs
    ///
    /// # Arguments
    /// * `factory_code_id` - Code ID for the factory contract
    /// * `minter_code_id` - Code ID for the minter contract
    /// * `collection_code_id` - Code ID for the collection contract
    pub fn setup_factory(
        &mut self,
        factory_code_id: u64,
        minter_code_id: u64,
        collection_code_id: u64,
    ) -> Result<(Addr, AppResponse)> {
        let factory_creator = self.users.factory_contract_creator();
        let (addr, response) = self.factory.instantiate(
            &mut self.app,
            factory_code_id,
            minter_code_id,
            collection_code_id,
            &factory_creator.address,
        )?;
        Ok((addr, response))
    }

    /// Creates a new minter instance using the factory
    ///
    /// # Returns
    /// * `Result<(Addr, Addr, AppResponse)>` - (minter address, sg721 address, response)
    pub fn create_minter(&mut self) -> Result<(Addr, Addr, AppResponse)> {
        let creator = self.users.tile_contract_creator();
        let (minter_addr, sg721_addr, response) = self
            .factory
            .create_test_minter(&mut self.app, &creator.address)?;

        self.minter = MinterContract::new(minter_addr.clone());
        self.tiles = TilesContract::new(sg721_addr.clone());

        Ok((minter_addr, sg721_addr, response))
    }

    /// Performs complete setup of all contracts
    ///
    /// # Returns
    /// * `Result<(Self, AppResponse)>` - The configured launchpad and setup response
    pub fn setup() -> Result<(Self, AppResponse)> {
        let mut launchpad = Self::new_empty();
        let (factory_id, minter_id, collection_id) = launchpad.store_contracts()?;
        let (_, _) = launchpad.setup_factory(factory_id, minter_id, collection_id)?;

        // Set block time to after genesis mint start time
        launchpad.app.set_genesis_time();

        let (_, _, response) = launchpad.create_minter()?;
        launchpad.app.advance_time(2 * 86400); // Advance 2 days
        Ok((launchpad, response))
    }
}
