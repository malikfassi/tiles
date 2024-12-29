use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::Contract;
use sg_multi_test::StargazeApp;
use sg_std::StargazeMsgWrapper;
use sg_std::GENESIS_MINT_START_TIME;

pub struct TestApp {
    app: StargazeApp,
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}

impl TestApp {
    pub fn new() -> Self {
        let mut app = Self {
            app: StargazeApp::default(),
        };
        app.set_genesis_time();
        app
    }

    pub fn inner(&self) -> &StargazeApp {
        &self.app
    }

    pub fn inner_mut(&mut self) -> &mut StargazeApp {
        &mut self.app
    }

    pub fn set_genesis_time(&mut self) {
        let mut block = self.app.block_info();
        block.time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
        self.app.set_block(block);
    }

    pub fn advance_time(&mut self, seconds: u64) {
        let mut block = self.app.block_info();
        block.time = block.time.plus_seconds(seconds);
        self.app.set_block(block);
    }

    pub fn store_code(&mut self, contract: Box<dyn Contract<StargazeMsgWrapper>>) -> u64 {
        self.app.store_code(contract)
    }

    pub fn get_balance(&self, addr: &Addr, denom: &str) -> Option<u128> {
        self.app
            .wrap()
            .query_balance(addr.to_string(), denom)
            .ok()
            .map(|c| c.amount.u128())
    }

    pub fn fund_account(&mut self, addr: &Addr, amount: u128, denom: &str) -> anyhow::Result<()> {
        self.app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, addr, vec![Coin::new(amount, denom)])
        })
    }
}
