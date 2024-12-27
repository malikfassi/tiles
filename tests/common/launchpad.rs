use crate::common::{
    app::TestApp,
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    users::TestUsers,
};

pub struct Launchpad {
    pub app: TestApp,
    pub users: TestUsers,
    pub factory: FactoryContract,
    pub minter: MinterContract,
    pub tiles: TilesContract,
}

impl Launchpad {
    pub fn new() -> Self {
        let mut app = TestApp::new();
        app.set_genesis_time();
        let users = TestUsers::new();

        // Fund all accounts
        users.fund_all(&mut app);

        // Store contract codes
        let collection_code_id = TilesContract::store_code(&mut app).unwrap();
        let minter_code_id = MinterContract::store_code(&mut app).unwrap();
        let factory_code_id = FactoryContract::store_code(&mut app).unwrap();

        // Setup factory
        let mut factory = FactoryContract::new(&mut app, "factory");
        let factory_creator = users.factory_contract_creator();
        factory
            .instantiate(
                &mut app,
                factory_code_id,
                minter_code_id,
                collection_code_id,
                &factory_creator,
            )
            .unwrap();

        // Setup collection and create minter
        let creator = users.tile_contract_creator();
        let (minter_addr, tiles_addr) = factory
            .create_test_minter(&mut app, &creator, collection_code_id)
            .unwrap();

        // Advance time past mint start time
        app.advance_time(2 * 86400); // Advance 2 days

        Self {
            app,
            users,
            tiles: TilesContract::new(tiles_addr),
            factory,
            minter: MinterContract::new(minter_addr),
        }
    }
}
