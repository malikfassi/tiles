use anyhow::Error as AnyhowError;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::Executor;
use sg2::msg::CollectionParams;
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sg_multi_test::StargazeApp;
use vending_factory::msg::{VendingMinterCreateMsg, VendingMinterInitMsgExtension};

use crate::common::{
    mock::mock_app, tiles_contract::TilesContract, vending_factory::VendingFactoryContract,
    NATIVE_DENOM,
};

pub struct TestSetup {
    pub app: StargazeApp,
    pub sender: Addr,
    pub factory: VendingFactoryContract,
    pub tiles: TilesContract,
}

#[derive(Debug)]
pub struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TestError {}

impl From<AnyhowError> for TestError {
    fn from(err: AnyhowError) -> Self {
        TestError(err.to_string())
    }
}

pub fn setup_test() -> Result<TestSetup, Box<dyn std::error::Error>> {
    println!("Setting up test environment...");
    let (mut app, sender) = mock_app();
    println!("Created mock app with sender: {}", sender);

    // Create factory
    println!("Creating vending factory...");
    let mut factory = VendingFactoryContract::new(&mut app, &sender);
    println!("Created factory at address: {}", factory.contract_addr);

    // Get current block time
    let start_time = app.block_info().time;
    println!("Current block time: {}", start_time);

    // Create minter
    println!("Creating minter...");
    let create_msg = VendingMinterCreateMsg {
        init_msg: VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: None,
            start_time,
            num_tokens: 100,
            mint_price: Coin::new(100_000_000, NATIVE_DENOM),
            per_address_limit: 3,
            whitelist: None,
        },
        collection_params: CollectionParams {
            code_id: factory.sg721_code_id,
            name: "Test Tiles".to_string(),
            symbol: "TEST".to_string(),
            info: CollectionInfo::<RoyaltyInfoResponse> {
                creator: sender.to_string(),
                description: "Test collection".to_string(),
                image: "ipfs://test.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
        },
    };
    println!("Creating minter with params: {:?}", create_msg);

    let (minter_addr, collection_addr) = factory.create_minter(&mut app, &sender, create_msg)?;
    println!("Created minter at address: {}", minter_addr);
    println!("Created collection at address: {}", collection_addr);

    // Create tiles contract instance
    println!("Creating tiles contract instance...");
    let tiles = TilesContract::new(collection_addr.clone());
    println!("Created tiles contract instance");

    // Check sender balance before minting
    let balance = app.wrap().query_balance(sender.as_str(), NATIVE_DENOM)?;
    println!("Sender balance before minting: {}", balance.amount);

    // Mint a token
    println!("Minting token...");
    let mint_result = app.execute_contract(
        sender.clone(),
        minter_addr.clone(),
        &vending_minter::msg::ExecuteMsg::Mint {},
        &[Coin::new(100_000_000, NATIVE_DENOM)],
    );

    let _res = match mint_result {
        Ok(res) => {
            println!("Token minted successfully");
            println!("Mint events:");
            for event in &res.events {
                println!("Event type: {}", event.ty);
                for attr in &event.attributes {
                    println!("  {}: {}", attr.key, attr.value);
                }
            }
            res
        }
        Err(e) => {
            println!("Failed to mint token: {:?}", e);
            return Err(Box::new(TestError::from(e)));
        }
    };

    Ok(TestSetup {
        app,
        sender,
        factory,
        tiles,
    })
}
