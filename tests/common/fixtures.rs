use cosmwasm_std::{Addr, Coin};
use sg_multi_test::StargazeApp;
use cw_multi_test::Executor;
use vending_factory::msg::{VendingMinterCreateMsg, VendingMinterInitMsgExtension};
use sg2::msg::CollectionParams;
use sg721::CollectionInfo;


use crate::common::{
    mock::mock_app,
    tiles_contract::TilesContract,
    vending_factory::VendingFactoryContract,
    NATIVE_DENOM,
};

pub struct TestSetup {
    pub app: StargazeApp,
    pub sender: Addr,
    pub factory: VendingFactoryContract,
    pub tiles: TilesContract,
}

pub fn setup_test() -> Result<TestSetup, Box<dyn std::error::Error>> {
    let (mut app, sender) = mock_app();

    // Create factory
    let mut factory = VendingFactoryContract::new(&mut app, &sender);

    // Get current block time
    let start_time = app.block_info().time;

    // Create minter
    let (minter_addr, collection_addr) = factory
        .create_minter(
            &mut app,
            &sender,
            VendingMinterCreateMsg {
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
            },
        )?;

    // Create tiles contract instance
    let tiles = TilesContract::new(&mut app, &sender, &factory, collection_addr);

    // Mint a token
    app.execute_contract(
        sender.clone(),
        minter_addr,
        &vending_minter::msg::ExecuteMsg::Mint {},
        &[Coin::new(100_000_000, NATIVE_DENOM)],
    )?;

    Ok(TestSetup {
        app,
        sender,
        factory,
        tiles,
    })
} 