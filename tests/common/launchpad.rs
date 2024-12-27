use cosmwasm_std::{Addr, Coin, Decimal};
use cw_multi_test::ContractWrapper;
use sg2::msg::CollectionParams;
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sg_std::GENESIS_MINT_START_TIME;
use sg_std::NATIVE_DENOM;
use tiles::contract::{
    execute::execute_handler, instantiate::instantiate_handler, query::query_handler,
};
use tiles::defaults::constants::{MAX_PER_ADDRESS_LIMIT, MINT_PRICE};
use vending_factory::msg::VendingMinterInitMsgExtension;

use crate::common::{
    app::TestApp,
    contracts::{factory::FactoryContract, minter::MinterContract, tiles::TilesContract},
    users::test_users::TestUsers,
    users::UserRole,
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
        // Store collection contract code
        let collection_contract = Box::new(ContractWrapper::new(
            execute_handler,
            instantiate_handler,
            query_handler,
        ));
        let collection_code_id = app.store_code(collection_contract);

        // Store minter code
        let minter_code_id = MinterContract::store_code(app).unwrap();

        // Store and instantiate factory
        let mut factory = FactoryContract::new(app, "factory");
        let factory_code_id = factory.store_code(app).unwrap();
        let _factory_addr = factory
            .instantiate(
                app,
                factory_code_id,
                minter_code_id,
                collection_code_id,
                &users.factory_contract_creator(),
            )
            .unwrap();

        // Setup collection params
        let collection_params = CollectionParams {
            code_id: collection_code_id,
            name: "Test Collection".to_string(),
            symbol: "TEST".to_string(),
            info: CollectionInfo {
                creator: users.get(UserRole::Owner).address.to_string(),
                description: "Test Collection".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: Some(RoyaltyInfoResponse {
                    payment_address: users.get(UserRole::Owner).address.to_string(),
                    share: Decimal::percent(5),
                }),
            },
        };

        // Setup minter init message
        let block_time = app.inner().block_info().time;
        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: Some(users.get(UserRole::Owner).address.to_string()),
            start_time: block_time.plus_seconds(86400), // Start in 1 day
            num_tokens: 100,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: MAX_PER_ADDRESS_LIMIT,
            whitelist: None,
        };

        // Create minter through factory
        let res = factory
            .create_minter(
                app,
                &users.get(UserRole::Owner).address,
                collection_params,
                init_msg,
            )
            .unwrap();

        // Extract contract addresses
        let minter_addr = res
            .events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| {
                let code_id_attr = e.attributes.iter().find(|a| a.key == "code_id")?;
                if code_id_attr.value == "2" {
                    e.attributes.iter().find(|a| a.key == "_contract_addr")
                } else {
                    None
                }
            })
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("Minter address not found in events");

        let sg721_addr = res
            .events
            .iter()
            .find(|e| {
                e.ty == "wasm"
                    && e.attributes
                        .iter()
                        .any(|a| a.key == "action" && a.value == "instantiate_sg721_reply")
            })
            .and_then(|e| e.attributes.iter().find(|a| a.key == "sg721_address"))
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("SG721 address not found in events");

        // Set block time to after minting start time
        app.advance_time(2 * 86400); // Advance 2 days

        (minter_addr, sg721_addr)
    }

    // Helper methods with role-based access
    pub fn mint_as(&mut self, role: UserRole) -> u32 {
        let address = self.users.get(role).address.clone();
        self.mint_token(&address)
    }

    // Low-level helper methods
    fn mint_token(&mut self, owner: &Addr) -> u32 {
        self.minter.mint(&mut self.app, owner).unwrap()
    }

    // Assertions
    pub fn assert_token_owner(&self, token_id: u32, expected_role: UserRole) {
        let expected_owner = self.users.get(expected_role).address.clone();
        let actual_owner = self.tiles.query_token_owner(&self.app, token_id).unwrap();
        assert_eq!(actual_owner, expected_owner);
    }

    pub fn assert_balance(&self, role: UserRole, expected_balance: u128) {
        self.users.assert_balance(&self.app, role, expected_balance);
    }
}
