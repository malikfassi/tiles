use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::{Contract, Executor};
use sg_multi_test::StargazeApp;
use sg_std::StargazeMsgWrapper;
use sg_std::GENESIS_MINT_START_TIME;

pub struct TestApp {
    app: StargazeApp,
}

impl TestApp {
    pub fn new() -> Self {
        Self {
            app: StargazeApp::default(),
        }
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

    pub fn inner(&self) -> &StargazeApp {
        &self.app
    }

    pub fn inner_mut(&mut self) -> &mut StargazeApp {
        &mut self.app
    }

    pub fn get_balance(&self, address: &Addr, denom: &str) -> Option<u128> {
        self.app
            .wrap()
            .query_balance(address.to_string(), denom)
            .ok()
            .map(|c| c.amount.u128())
    }

    pub fn instantiate_contract<T>(
        &mut self,
        code_id: u64,
        sender: Addr,
        msg: &T,
        funds: &[Coin],
        label: &str,
        admin: Option<String>,
    ) -> anyhow::Result<Addr>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        self.app
            .instantiate_contract(code_id, sender, msg, funds, label, admin)
    }

    pub fn execute_contract<T>(
        &mut self,
        sender: Addr,
        contract: Addr,
        msg: &T,
        funds: &[Coin],
    ) -> anyhow::Result<cw_multi_test::AppResponse>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        self.app.execute_contract(sender, contract, msg, funds)
    }
}

impl Default for TestApp {
    fn default() -> Self {
        let mut app = Self::new();
        app.set_genesis_time();
        app
    }
}
