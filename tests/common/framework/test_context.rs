use super::scenario::Scenario;
use crate::common::app::TestApp;
use crate::common::contracts::{TilesContract, VendingContract};
use crate::common::users::{TestUsers, UserRole};
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::ContractWrapper;
use sg2::msg::CollectionParams;
use sg721::CollectionInfo;
use sg_std::NATIVE_DENOM;
use tiles::contract::{
    execute::execute_handler, instantiate::instantiate_handler, query::query_handler,
};
use tiles::defaults::constants::MINT_PRICE;
use vending_factory::msg::VendingMinterInitMsgExtension;

pub struct TestContext {
    pub app: TestApp,
    pub tiles: TilesContract,
    pub vending: VendingContract,
    pub minter: Addr,
    pub users: TestUsers,
    pub scenario: Scenario,
}

impl TestContext {
    pub fn new() -> Self {
        let mut app = TestApp::default();
        let users = TestUsers::default();

        // Fund all users
        users.fund_all(&mut app);

        // Store and instantiate contracts
        let (minter_addr, tiles_addr) = Self::setup_contracts(&mut app, &users);

        // Create vending contract
        let vending = VendingContract::new(&mut app, "vending");

        Self {
            app,
            tiles: TilesContract::new(tiles_addr),
            vending,
            minter: minter_addr,
            users,
            scenario: Scenario::default(),
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

        // Store vending minter contract code
        let minter_contract = Box::new(
            ContractWrapper::new(
                vending_minter::contract::execute,
                vending_minter::contract::instantiate,
                vending_minter::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        let minter_code_id = app.store_code(minter_contract);

        // Store and instantiate vending factory
        let mut vending = VendingContract::new(app, "vending");
        let factory_code_id = vending.store_code(app).unwrap();
        let _vending_addr = vending
            .instantiate(app, factory_code_id, minter_code_id, collection_code_id)
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
                royalty_info: None,
            },
        };

        // Setup minter init message
        let block_time = app.inner().block_info().time;
        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: None,
            start_time: block_time.plus_seconds(86400), // Start in 1 day
            num_tokens: 100,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: 3,
            whitelist: None,
        };

        // Create minter
        let res = vending
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

    pub fn update_pixel_as(&mut self, role: UserRole, token_id: u32, color: &str) {
        let address = self.users.get(role).address.clone();
        self.update_pixel(&address, token_id, color)
    }

    // Low-level helper methods
    fn mint_token(&mut self, owner: &Addr) -> u32 {
        let token_id = self
            .tiles
            .mint_token(&mut self.app, owner, &self.minter)
            .unwrap();

        self.scenario.record_mint(owner.clone(), token_id);
        token_id
    }

    fn update_pixel(&mut self, owner: &Addr, token_id: u32, color: &str) {
        self.tiles
            .update_pixel(&mut self.app, owner, token_id, color.to_string())
            .unwrap();

        self.scenario
            .record_pixel_update(owner.clone(), token_id, 0, color.to_string());
    }

    // Assertions
    pub fn assert_token_owner(&self, token_id: u32, expected_role: UserRole) {
        let expected_owner = self.users.get(expected_role).address.clone();
        let actual_owner = self
            .scenario
            .get_token_owner(token_id)
            .expect("Token not found");
        assert_eq!(actual_owner, &expected_owner);
    }

    pub fn assert_balance(&self, role: UserRole, expected_balance: u128) {
        self.users.assert_balance(&self.app, role, expected_balance);
    }
}
