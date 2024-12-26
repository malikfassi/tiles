use crate::common::contracts::{tiles::TilesContract, vending::VendingContract};
use crate::common::helpers::users::TestUsers;
use crate::common::test_module::TilesApp as App;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::ContractWrapper;
use sg2::msg::CollectionParams;
use sg721::CollectionInfo;
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};
use tiles::contract::{
    execute::execute_handler, instantiate::instantiate_handler, query::query_handler,
};
use tiles::defaults::constants::MINT_PRICE;
use vending_factory::msg::VendingMinterInitMsgExtension;

pub enum UserType {
    Buyer,
    Whale,
    Poor,
    Owner,
}

impl UserType {
    fn as_str(&self) -> &'static str {
        match self {
            UserType::Buyer => "buyer",
            UserType::Whale => "whale",
            UserType::Poor => "poor",
            UserType::Owner => "owner",
        }
    }
}

pub struct TestSetup {
    pub app: App,
    pub tiles: TilesContract,
    pub vending: VendingContract,
    pub minter: Addr,
    pub users: TestUsers,
    pub scenario: Scenario,
}

pub struct Scenario {
    pub minted_tokens: Vec<(Addr, u32)>, // (owner, token_id)
}

impl Scenario {
    fn new() -> Self {
        Self {
            minted_tokens: Vec::new(),
        }
    }
}

impl TestSetup {
    fn setup_block_time(app: &mut App) {
        let mut block = app.block_info();
        let new_time = GENESIS_MINT_START_TIME + 2 * 86_400_000_000_000_u64;
        block.time = Timestamp::from_nanos(new_time);
        app.set_block(block);
    }

    fn store_contract_codes(app: &mut App) -> (u64, u64, u64) {
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

        // Store vending factory contract code
        let vending = VendingContract::new(app, "vending");
        let factory_code_id = vending.store_code(app).unwrap();

        (collection_code_id, minter_code_id, factory_code_id)
    }

    fn setup_vending_minter(
        app: &mut App,
        vending: &mut VendingContract,
        users: &TestUsers,
        collection_code_id: u64,
        minter_code_id: u64,
        factory_code_id: u64,
    ) -> (Addr, Addr) {
        let _vending_addr = vending
            .instantiate(app, factory_code_id, minter_code_id, collection_code_id)
            .unwrap();

        let collection_params = CollectionParams {
            code_id: collection_code_id,
            name: "Test Collection".to_string(),
            symbol: "TEST".to_string(),
            info: CollectionInfo {
                creator: users.collection_owner.address.to_string(),
                description: "Test Collection".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
        };

        let block_time = app.block_info().time;
        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: None,
            start_time: Timestamp::from_nanos(block_time.nanos() + 86_400_000_000_000_u64),
            num_tokens: 100,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: 3,
            whitelist: None,
        };

        let res = vending
            .create_minter(
                app,
                &users.collection_owner.address,
                collection_params,
                init_msg,
            )
            .unwrap();

        // Extract contract addresses from events
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

        (minter_addr, sg721_addr)
    }

    pub fn new() -> Self {
        let mut app = App::default();
        let users = TestUsers::default();
        
        // Setup initial state
        Self::setup_block_time(&mut app);
        users.fund_all(&mut app);

        // Store contract codes
        let (collection_code_id, minter_code_id, factory_code_id) = Self::store_contract_codes(&mut app);

        // Setup vending minter and get contract addresses
        let mut vending = VendingContract::new(&mut app, "vending");
        let (minter_addr, sg721_addr) = Self::setup_vending_minter(
            &mut app,
            &mut vending,
            &users,
            collection_code_id,
            minter_code_id,
            factory_code_id,
        );

        // Set block time to after minting start time
        let mut block = app.block_info();
        let new_time = block.time.plus_seconds(2 * 86400);
        block.time = new_time;
        app.set_block(block);

        Self {
            app,
            tiles: TilesContract::new(sg721_addr),
            vending,
            minter: minter_addr,
            users,
            scenario: Scenario::new(),
        }
    }

    // Helper methods for common operations
    pub fn mint_token(&mut self, owner: &Addr) -> u32 {
        // Mint the token and get the response
        let mint_response = self.tiles
            .mint_through_minter(&mut self.app, owner, &self.minter)
            .unwrap();

        // Extract token_id from the response events
        let token_id = mint_response
            .events
            .iter()
            .find(|e| e.ty == "wasm")
            .and_then(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "token_id")
                    .map(|a| a.value.parse::<u32>().unwrap())
            })
            .expect("Token ID not found in mint response");

        // Store the minting information
        self.scenario.minted_tokens.push((owner.clone(), token_id));
        
        token_id
    }

    pub fn update_pixel(&mut self, owner: &Addr, token_id: u32, color: &str) {
        self.tiles
            .update_pixel(&mut self.app, owner, token_id, color.to_string())
            .unwrap();
    }

    pub fn get_user_addr(&self, user_type: UserType) -> Addr {
        match user_type {
            UserType::Buyer => self.users.regular_buyer.address.clone(),
            UserType::Whale => self.users.whale_buyer.address.clone(),
            UserType::Poor => self.users.poor_buyer.address.clone(),
            UserType::Owner => self.users.collection_owner.address.clone(),
        }
    }

    pub fn mint_as(&mut self, user_type: UserType) -> u32 {
        let user_addr = self.get_user_addr(user_type);
        self.mint_token(&user_addr)
    }

    pub fn update_pixel_as(&mut self, user_type: UserType, token_id: u32, color: &str) {
        let user_addr = self.get_user_addr(user_type);
        self.update_pixel(&user_addr, token_id, color);
    }

    // Add more helper methods as needed...
}
