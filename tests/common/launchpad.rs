use cosmwasm_std::Addr;
use crate::common::{
    app::TestApp,
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    users::test_users::{TestUsers, User},
};

pub struct Launchpad {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub factory: FactoryContract,
    pub minter: MinterContract,
}

impl Launchpad {
    pub fn new() -> Self {
        let mut app = TestApp::new();
        app.set_genesis_time();
        let users = TestUsers::default();

        // Fund all users
        users.fund_all_accounts(&mut app);

        // Setup contracts
        let (minter_addr, tiles_addr) = Self::setup_contracts(&mut app, &users);

        // Create factory contract
        let factory = FactoryContract::new(&mut app, "factory");

        Self {
            app,
            users,
            tiles: TilesContract::new(tiles_addr),
            factory,
            minter: MinterContract::new(minter_addr),
        }
    }

    fn setup_contracts(app: &mut TestApp, users: &TestUsers) -> (Addr, Addr) {
        println!("\n=== Setting up contracts ===");
        
        // Store contract codes
        let collection_code_id = TilesContract::store_code(app).unwrap();
        let minter_code_id = MinterContract::store_code(app).unwrap();
        let factory_code_id = FactoryContract::store_code(app).unwrap();
        
        // Setup factory
        let mut factory = FactoryContract::new(app, "factory");
        let factory_creator = users.factory_contract_creator();
        factory.instantiate(
            app,
            factory_code_id,
            minter_code_id,
            collection_code_id,
            &factory_creator,
        ).unwrap();

        // Setup collection and create minter
        let creator = users.tile_contract_creator();
        let (minter_addr, sg721_addr) = factory.create_test_minter(
            app,
            &creator,
            collection_code_id,
        ).unwrap();

        // Set block time to after minting start time
        app.advance_time(2 * 86400); // Advance 2 days

        (minter_addr, sg721_addr)
    }

    // Assertions
    pub fn assert_token_owner(&self, token_id: u32, expected_owner: &User) {
        let actual_owner = self.tiles.query_token_owner(&self.app, token_id).unwrap();
        assert_eq!(actual_owner, expected_owner.address);
    }
}
