use cosmwasm_std::{Addr, Coin, Timestamp};
use sg_multi_test::StargazeApp;

use crate::defaults::config::DEFAULT_INITIAL_BALANCE;
use crate::defaults::constants::NATIVE_DENOM;

pub fn mock_app() -> (StargazeApp, Addr) {
    let mut app = StargazeApp::default();
    let sender = Addr::unchecked("owner");

    // Set block time (2023-01-01)
    let mut block = app.block_info();
    block.time = Timestamp::from_seconds(1672531200);
    app.set_block(block);

    // Fund sender
    app.init_modules(|router, _, storage| {
        router.bank.init_balance(
            storage,
            &sender,
            vec![Coin::new(DEFAULT_INITIAL_BALANCE, NATIVE_DENOM)],
        )
    })
    .expect("Failed to init modules");

    (app, sender)
}
