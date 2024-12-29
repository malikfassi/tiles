use anyhow::Result;
use cosmwasm_std::{Addr, Timestamp};
use cw_multi_test::Contract;
use sg_multi_test::StargazeApp;
use sg_std::{StargazeMsgWrapper, GENESIS_MINT_START_TIME};

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

    pub fn get_balance(&self, addr: &Addr, denom: &str) -> Result<u128> {
        Ok(self.app.wrap().query_balance(addr, denom)?.amount.u128())
    }

    pub fn store_code(&mut self, contract: Box<dyn Contract<StargazeMsgWrapper>>) -> u64 {
        self.app.store_code(contract)
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
}
