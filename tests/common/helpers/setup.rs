use crate::common::contracts::tiles::TilesContract;
use crate::common::contracts::vending::VendingFactory;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::ContractWrapper;
use sg2::msg::{CollectionParams, CreateMinterMsg};
use sg721::CollectionInfo;
use sg721_base::entry::{
    execute as sg721_execute, instantiate as sg721_instantiate, query as sg721_query,
};
use sg_multi_test::StargazeApp as App;
use sg_std::NATIVE_DENOM;
use vending_minter::contract::{
    execute as minter_execute, instantiate as minter_instantiate, query as minter_query,
};

pub struct TestSetup {
    pub app: App,
    pub admin: Addr,
    pub factory: VendingFactory,
    pub minter: Addr,
    pub collection: TilesContract,
}

impl TestSetup {
    pub fn new() -> Self {
        println!("\n=== Setting up test environment ===");
        // Set up app with initial balances
        let mut app = App::default();

        // Set block time to a known value (2024-01-01 00:00:00 UTC)
        let block_time = Timestamp::from_seconds(1704067200);
        println!("Setting block time to: {}", block_time);
        app.set_block(cosmwasm_std::BlockInfo {
            height: 12345,
            time: block_time,
            chain_id: "testing".to_string(),
        });

        // Set initial balance for admin
        let admin = Addr::unchecked("admin");
        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &admin,
                    vec![Coin::new(10_000_000_000, NATIVE_DENOM)],
                )
                .unwrap();
        });
        println!(
            "✓ App initialized with admin: {} and balance: {} {}",
            admin, 10_000_000_000, NATIVE_DENOM
        );

        // Store contract codes
        println!("\nStoring contract codes...");
        let minter_code_id = app.store_code(Box::new(ContractWrapper::new(
            minter_execute,
            minter_instantiate,
            minter_query,
        )));
        println!("✓ Stored minter code with ID: {}", minter_code_id);

        let collection_code_id = app.store_code(Box::new(ContractWrapper::new(
            sg721_execute,
            sg721_instantiate,
            sg721_query,
        )));
        println!("✓ Stored collection code with ID: {}", collection_code_id);

        // Create factory
        let factory = VendingFactory::new(&mut app, &admin, minter_code_id, collection_code_id);

        // Create minter with start time 1 hour after block time
        let start_time = block_time.plus_seconds(3600);
        println!("\nCreating minter configuration...");
        println!("- Start time: {}", start_time);
        println!("- Start time (seconds): {}", start_time.seconds());

        let msg = CreateMinterMsg {
            init_msg: vending_factory::msg::VendingMinterInitMsgExtension {
                base_token_uri: "ipfs://QmYxw1rURvnbQbBRTfmVaZtxSrkrfsbodNzibgBrVrUrtN".to_string(),
                payment_address: None,
                start_time: start_time,
                num_tokens: 100,
                mint_price: Coin::new(100_000, NATIVE_DENOM),
                per_address_limit: 3,
                whitelist: None,
            },
            collection_params: CollectionParams {
                code_id: collection_code_id,
                name: "Test Collection".to_string(),
                symbol: "TEST".to_string(),
                info: CollectionInfo {
                    creator: admin.to_string(),
                    description: "Test Collection".to_string(),
                    image: "ipfs://image".to_string(),
                    external_link: None,
                    royalty_info: None,
                    explicit_content: Some(false),
                    start_trading_time: Some(start_time.plus_seconds(60 * 60 * 24)), // 1 day after start
                },
            },
        };

        println!("\nCreating minter with params:");
        println!("- Base token URI: {}", msg.init_msg.base_token_uri);
        println!("- Start time: {}", msg.init_msg.start_time);
        println!("- Num tokens: {}", msg.init_msg.num_tokens);
        println!(
            "- Mint price: {} {}",
            msg.init_msg.mint_price.amount, msg.init_msg.mint_price.denom
        );
        println!("- Per address limit: {}", msg.init_msg.per_address_limit);
        println!("Collection params:");
        println!("- Code ID: {}", msg.collection_params.code_id);
        println!("- Name: {}", msg.collection_params.name);
        println!("- Symbol: {}", msg.collection_params.symbol);
        println!("- Creator: {}", msg.collection_params.info.creator);
        println!(
            "- Start trading time: {:?}",
            msg.collection_params.info.start_trading_time
        );

        let (minter, collection) = match factory.create_minter(&mut app, &admin, msg) {
            Ok((m, c)) => {
                println!("✓ Successfully created minter at address: {}", m);
                println!("✓ Successfully created collection at address: {}", c.addr);
                (m, c)
            }
            Err(e) => {
                println!("❌ Failed to create minter and collection");
                println!("Error details: {:#?}", e);
                panic!("Failed to create minter: {}", e);
            }
        };

        println!("=== Test environment setup complete ===\n");

        Self {
            app,
            admin,
            factory,
            minter,
            collection,
        }
    }
}
