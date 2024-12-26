use crate::common::{
    constants::{MINT_PRICE, NATIVE_DENOM, CREATION_FEE},
    contracts::{tiles::TilesContract, vending::VendingContract},
    test_module::TilesApp as App,
};
use cosmwasm_std::{Addr, Coin, Timestamp, BlockInfo};
use cw_multi_test::ContractWrapper;
use sg2::msg::CollectionParams;
use sg721::CollectionInfo;
use sg_std::GENESIS_MINT_START_TIME;
use tiles::contract::{
    execute::execute_handler, instantiate::instantiate_handler, query::query_handler,
};
use vending_factory::msg::VendingMinterInitMsgExtension;

pub struct TestSetup {
    pub app: App,
    pub tiles: TilesContract,
    pub vending: VendingContract,
    pub minter: Addr,
}

impl TestSetup {
    pub fn new() -> Self {
        println!("Creating new test setup...");
        let mut app = App::default();
        
        // Initialize app with genesis time
        let mut block = app.block_info();
        println!("Initial block info - Height: {}, Time: {}, Chain ID: {}", 
            block.height, block.time, block.chain_id);
        
        block.time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
        println!("Setting genesis time to: {} ({} nanos)", 
            GENESIS_MINT_START_TIME / 1_000_000_000u64, GENESIS_MINT_START_TIME);
        
        app.set_block(block);
        println!("Block time after genesis set: {}", app.block_info().time);

        // Set block time to genesis + 2 days
        let mut block = app.block_info();
        let new_time = GENESIS_MINT_START_TIME + 2 * 86400_000_000_000u64;
        block.time = Timestamp::from_nanos(new_time);
        println!("Setting block time to genesis + 2 days: {} ({} nanos)", 
            block.time, new_time);
        app.set_block(block);
        println!("Block time after update: {}", app.block_info().time);

        // Fund creator's account
        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked("creator"),
                    vec![Coin::new(1_000_000_000, NATIVE_DENOM)],
                )
                .unwrap();
        });
        println!("Funded creator account with {} {}", 1_000_000_000, NATIVE_DENOM);

        // Store collection contract code
        let collection_contract = Box::new(ContractWrapper::new(
            execute_handler,
            instantiate_handler,
            query_handler,
        ));
        let collection_code_id = app.store_code(collection_contract);
        println!("Stored collection contract with code ID: {}", collection_code_id);

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
        println!("Stored vending minter contract with code ID: {}", minter_code_id);

        // Store vending factory contract code
        let mut vending = VendingContract::new(&mut app, "vending");
        let factory_code_id = vending.store_code(&mut app).unwrap();
        println!("Stored vending factory contract with code ID: {}", factory_code_id);

        let _vending_addr = vending
            .instantiate(
                &mut app,
                factory_code_id,
                minter_code_id,
                collection_code_id,
            )
            .unwrap();
        println!("Instantiated vending factory at address: {}", _vending_addr);

        let collection_params = CollectionParams {
            code_id: collection_code_id,
            name: "Test Collection".to_string(),
            symbol: "TEST".to_string(),
            info: CollectionInfo {
                creator: "creator".to_string(),
                description: "Test Collection".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
        };
        println!("Created collection params with code ID: {}", collection_code_id);

        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: None,
            start_time: Timestamp::from_nanos(new_time + 86400_000_000_000u64), // Set start time to current block time + 1 day
            num_tokens: 100,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: 3,
            whitelist: None,
        };
        println!("Created init msg with start time: {} (block time + 1 day)", init_msg.start_time);

        println!("Current block time before create_minter: {}", app.block_info().time);
        let res = vending
            .create_minter(
                &mut app,
                &Addr::unchecked("creator"),
                collection_params,
                init_msg,
            )
            .unwrap();

        println!("\nDEBUG: All events from create_minter response:");
        for (idx, event) in res.events.iter().enumerate() {
            println!("Event {}: type = {}", idx, event.ty);
            println!("  Attributes:");
            for attr in &event.attributes {
                println!("    {} = {}", attr.key, attr.value);
            }
        }

        // Extract minter address from events
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
        println!("Created minter at address: {}", minter_addr);

        // Extract sg721 address from events
        let sg721_addr = res
            .events
            .iter()
            .find(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "instantiate_sg721_reply"))
            .and_then(|e| e.attributes.iter().find(|a| a.key == "sg721_address"))
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("SG721 address not found in events");
        println!("Created sg721 at address: {}", sg721_addr);

        let tiles = TilesContract::new(sg721_addr);
        println!("Setup complete!");

        // Set block time to after minting start time
        let mut block = app.block_info();
        let new_time = block.time.plus_seconds(2 * 86400); // Add 2 more days to ensure we're past the start time
        block.time = new_time;
        println!("Setting block time to after mint start: {} ({} nanos)", 
            block.time, new_time.nanos());
        app.set_block(block);
        println!("Block time after final update: {}", app.block_info().time);

        Self {
            app,
            tiles,
            vending,
            minter: minter_addr,
        }
    }
}
