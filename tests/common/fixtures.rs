use cosmwasm_std::{Addr, Coin};
use cw_multi_test::App;
use sg721::CollectionInfo;

use crate::common::{
    mock::init_modules,
    tiles_contract::TilesContract,
    vending_factory::VendingFactory,
};
use crate::defaults::config::{
    DEFAULT_BASE_PRICE, DEFAULT_DEV_FEE, DEFAULT_MINT_PRICE, DEFAULT_PRICE_SCALING,
};
use crate::defaults::constants::{
    DEFAULT_COLLECTION_DESCRIPTION, DEFAULT_COLLECTION_IMAGE, NATIVE_DENOM,
};
use crate::msg::InstantiateMsg;
use crate::state::Config;

// Default message objects for testing
pub const DEFAULT_MINT_MSG: sg721::MintMsg<Option<Empty>> = sg721::MintMsg {
    token_id: String::new(),
    owner: String::new(),
    token_uri: None,
    extension: None,
};

pub const DEFAULT_INSTANTIATE_MSG: InstantiateMsg = InstantiateMsg {
    name: String::new(),
    symbol: String::new(),
    minter: String::new(),
    collection_info: sg721::CollectionInfo {
        creator: String::new(),
        description: DEFAULT_COLLECTION_DESCRIPTION.to_string(),
        image: DEFAULT_COLLECTION_IMAGE.to_string(),
        external_link: None,
        explicit_content: None,
        start_trading_time: None,
        royalty_info: None,
    },
    dev_fee_percent: DEFAULT_DEV_FEE,
    base_price: DEFAULT_BASE_PRICE,
    price_scaling: Some(DEFAULT_PRICE_SCALING),
};

pub const DEFAULT_CONFIG: Config = Config {
    admin: Addr::unchecked(""),
    minter: Addr::unchecked(""),
    collection_info: sg721::CollectionInfo {
        creator: String::new(),
        description: DEFAULT_COLLECTION_DESCRIPTION.to_string(),
        image: DEFAULT_COLLECTION_IMAGE.to_string(),
        external_link: None,
        explicit_content: None,
        start_trading_time: None,
        royalty_info: None,
    },
    dev_address: Addr::unchecked(""),
    dev_fee_percent: DEFAULT_DEV_FEE,
    base_price: DEFAULT_BASE_PRICE,
    price_scaling: Some(DEFAULT_PRICE_SCALING),
};

pub struct TestSetup {
    pub app: App,
    pub sender: Addr,
    pub tiles: TilesContract,
    pub factory: VendingFactory,
}

impl TestSetup {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (mut app, sender) = init_modules();

        // Create factory
        let factory = VendingFactory::new(&mut app, &sender);

        // Create collection
        let mint_price = Coin::new(DEFAULT_MINT_PRICE, NATIVE_DENOM);
        let (minter_addr, collection_addr) = factory.create_minter(
            &mut app,
            &sender,
            vending_factory::msg::CreateMinterMsg {
                init_msg: None,
                collection_params: vending_factory::state::CollectionParams {
                    code_id: 0, // Set by factory
                    name: "Test Collection".to_string(),
                    symbol: "TEST".to_string(),
                    max_supply: Some(100),
                    mint_price,
                    per_address_limit: 10,
                    start_time: None,
                    payment_address: None,
                    collection_info: CollectionInfo {
                        creator: sender.to_string(),
                        description: "Test Collection".to_string(),
                        image: "https://example.com/image.png".to_string(),
                        external_link: None,
                        explicit_content: None,
                        start_trading_time: None,
                        royalty_info: None,
                    },
                },
            },
        )?;

        // Query balance to ensure funds are available
        let balance = app.wrap().query_balance(sender.as_str(), NATIVE_DENOM)?;
        println!("Balance: {}", balance.amount);

        // Mint a token
        app.execute_contract(
            sender.clone(),
            minter_addr,
            &vending_minter::msg::ExecuteMsg::Mint {},
            &[Coin::new(DEFAULT_MINT_PRICE, NATIVE_DENOM)],
        )?;

        let tiles = TilesContract::new(collection_addr);

        Ok(Self {
            app,
            sender,
            tiles,
            factory,
        })
    }
}
