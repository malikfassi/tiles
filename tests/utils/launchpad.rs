use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::CREATION_FEE;

use crate::utils::{
    app::TestApp,
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    users::TestUsers,
};

pub struct Launchpad {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub factory: FactoryContract,
    pub minter: MinterContract,
}

impl Default for Launchpad {
    fn default() -> Self {
        Self::new()
    }
}

impl Launchpad {
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

    pub fn store_contracts(&mut self) -> Result<(u64, u64, u64)> {
        // Store contracts in the correct order to get expected code IDs:
        // factory = 1, sg721 = 2, minter = 3
        let factory_code_id = FactoryContract::store_code(&mut self.app)?;
        let collection_code_id = TilesContract::store_code(&mut self.app)?;
        let minter_code_id = MinterContract::store_code(&mut self.app)?;

        // Return in the order expected by setup_factory: (factory, minter, collection)
        Ok((factory_code_id, minter_code_id, collection_code_id))
    }

    pub fn setup_factory(
        &mut self,
        factory_code_id: u64,
        minter_code_id: u64,
        collection_code_id: u64,
    ) -> Result<(Addr, AppResponse)> {
        let mut factory = FactoryContract::new(&mut self.app, "factory");
        let factory_creator = self.users.factory_contract_creator();
        let (addr, response) = factory.instantiate(
            &mut self.app,
            factory_code_id,
            minter_code_id,
            collection_code_id,
            &factory_creator.address,
        )?;
        self.factory = factory;
        Ok((addr, response))
    }

    pub fn create_minter(&mut self) -> Result<(Addr, Addr, AppResponse)> {
        let creator = self.users.tile_contract_creator();
        let (minter_addr, sg721_addr, response) = self
            .factory
            .create_test_minter(&mut self.app, &creator.address)?;

        self.minter = MinterContract::new(minter_addr.clone());
        self.tiles = TilesContract::new(sg721_addr.clone());

        Ok((minter_addr, sg721_addr, response))
    }

    pub fn setup() -> Result<(Self, AppResponse)> {
        let mut launchpad = Self::new_empty();
        let (factory_id, minter_id, collection_id) = launchpad.store_contracts()?;
        let (_, _) = launchpad.setup_factory(factory_id, minter_id, collection_id)?;
        let (_, _, response) = launchpad.create_minter()?;

        launchpad.app.advance_time(2 * 86400); // Advance 2 days
        Ok((launchpad, response))
    }

    // Backward compatibility
    pub fn new() -> Self {
        Self::setup().unwrap().0
    }
}
