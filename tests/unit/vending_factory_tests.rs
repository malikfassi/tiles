use cosmwasm_std::Coin;
use vending_factory::msg::{VendingMinterCreateMsg, VendingMinterInitMsgExtension};
use sg2::msg::CollectionParams;
use sg721::CollectionInfo;
use cosmwasm_std::{Decimal, Uint128};
use serde_json::to_string;

use crate::common::{fixtures::{setup_test, TestSetup}, NATIVE_DENOM};

#[test]
fn test_create_minter() {
    let TestSetup { mut app, sender, mut factory, .. } = setup_test().unwrap();

    // Get current block time
    let start_time = app.block_info().time;

    // Create minter with valid params
    let msg = VendingMinterCreateMsg {
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
            info: CollectionInfo {
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

    let res = factory.create_minter(&mut app, &sender, msg);
    assert!(res.is_ok());
}

#[test]
fn test_create_minter_with_invalid_params() {
    let TestSetup { mut app, sender, mut factory, .. } = setup_test().unwrap();

    // Get current block time
    let start_time = app.block_info().time;

    // Try to create minter with invalid params
    let msg = VendingMinterCreateMsg {
        init_msg: VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: None,
            start_time,
            num_tokens: 0, // Invalid number of tokens
            mint_price: Coin::new(100_000_000, NATIVE_DENOM),
            per_address_limit: 3,
            whitelist: None,
        },
        collection_params: CollectionParams {
            code_id: factory.sg721_code_id,
            name: "Test Tiles".to_string(),
            symbol: "TEST".to_string(),
            info: CollectionInfo {
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

    let res = factory.create_minter(&mut app, &sender, msg);
    assert!(res.is_err());
}

#[test]
fn test_mint_token() {
    let TestSetup { mut app, sender, mut factory, tiles } = setup_test().unwrap();

    // Get current block time
    let start_time = app.block_info().time;

    // Create minter with valid params
    let msg = VendingMinterCreateMsg {
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
            info: CollectionInfo {
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

    let res = factory.create_minter(&mut app, &sender, msg);
    assert!(res.is_ok());

    // Query config to verify setup
    let config = tiles.query_config(&app).unwrap();
    assert_eq!(config.minter, factory.minter_addr);
} 