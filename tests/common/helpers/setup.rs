use crate::common::{
    constants::{MINT_PRICE, NATIVE_DENOM},
    contracts::{tiles::TilesContract, vending::VendingContract},
    test_module::TilesApp as App,
};
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::ContractWrapper;
use sg2::msg::CollectionParams;
use sg721::CollectionInfo;
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
        let mut app = App::default();

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
        let mut vending = VendingContract::new(&mut app, "vending");
        let factory_code_id = vending.store_code(&mut app).unwrap();
        let _vending_addr = vending
            .instantiate(
                &mut app,
                factory_code_id,
                minter_code_id,
                collection_code_id,
            )
            .unwrap();

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

        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: None,
            start_time: Timestamp::from_seconds(0),
            num_tokens: 100,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: 10,
            whitelist: None,
        };

        let res = vending
            .create_minter(
                &mut app,
                &Addr::unchecked("creator"),
                collection_params,
                init_msg,
            )
            .unwrap();

        // Extract minter address from events
        let minter_addr = res
            .events
            .iter()
            .find(|e| e.ty == "wasm")
            .and_then(|e| e.attributes.iter().find(|a| a.key == "minter"))
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("Minter address not found in events");

        let tiles = TilesContract::new(minter_addr.clone());

        Self {
            app,
            tiles,
            vending,
            minter: minter_addr,
        }
    }
}
