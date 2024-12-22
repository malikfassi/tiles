use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, AppBuilder};

use tiles::defaults::config::DEFAULT_INITIAL_BALANCE;
use tiles::defaults::constants::NATIVE_DENOM;

pub fn init_modules() -> (App, Addr) {
    // Create app with initial balance
    let mut app = AppBuilder::new().build(|router, _api, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("sender"),
                vec![Coin::new(DEFAULT_INITIAL_BALANCE, NATIVE_DENOM)],
            )
            .unwrap();
    });

    // Create sender address
    let sender = Addr::unchecked("sender");

    (app, sender)
}
