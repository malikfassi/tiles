use cosmwasm_std::{Addr, Coin, Empty, StdError, Timestamp};
use cw_multi_test::{ContractWrapper, Executor};
use sg721_base::ContractError;
use sg_multi_test::StargazeApp;
use sg_std::StargazeMsgWrapper;

use crate::common::{
    tiles_contract::TilesContract,
    vending_factory::{create_vending_factory, VendingFactory},
    NATIVE_DENOM,
};

pub struct TestSetup {
    pub app: StargazeApp,
    pub sender: Addr,
    pub tiles: TilesContract,
}

pub fn setup_test() -> anyhow::Result<TestSetup> {
    println!("Setting up test environment...");

    // Create mock app with initial balance
    let sender = Addr::unchecked("owner");
    let mut app = StargazeApp::new();
    app.init_modules(|router, api, storage| {
        // Initialize bank balance
        router.bank.init_balance(
            storage,
            &sender,
            vec![Coin::new(1_000_000_000, NATIVE_DENOM)],
        ).unwrap();
    });

    // Initialize contract data
    let block = app.block_info();
    app.set_block(block);

    // Set block time (2023-01-01)
    let mut block = app.block_info();
    block.time = Timestamp::from_seconds(1672531200);
    app.set_block(block);

    // Create vending factory
    let factory = create_vending_factory(&mut app, &sender)?;

    // Get current block time
    let start_time = app.block_info().time;
    println!("Current block time: {}", start_time);

    // Create minter and collection
    let (minter_addr, collection_addr) = factory.create_minter(
        &mut app,
        &sender,
        "Test Tiles".to_string(),
        "TILE".to_string(),
        "Test collection".to_string(),
        "ipfs://test.png".to_string(),
        "ipfs://test/".to_string(),
        100,
        Coin::new(100_000_000, NATIVE_DENOM),
        3,
        Some(start_time.seconds()),
        None,
        None,
        None,
    )?;

    // Mint a token using the minter
    println!("Minting token...");
    app.execute_contract(
        sender.clone(),
        minter_addr.clone(),
        &vending_minter::msg::ExecuteMsg::Mint {},
        &[Coin::new(100_000_000, NATIVE_DENOM)],
    )?;

    // Use collection address as tiles contract
    let tiles_addr = collection_addr;

    // Create tiles contract to interact with the collection
    let tiles = TilesContract::new(tiles_addr);

    Ok(TestSetup {
        app,
        sender,
        tiles,
    })
}

pub fn mock_tile_metadata() -> tiles::core::tile::metadata::TileMetadata {
    tiles::core::tile::metadata::TileMetadata {
        pixels: vec![],
    }
}
