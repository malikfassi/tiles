use cosmwasm_std::{Addr, Timestamp};
use cw_multi_test::{Contract, ContractWrapper, Executor};
use sg_multi_test::StargazeApp;
use sg_std::StargazeMsgWrapper;
use sg_std::GENESIS_MINT_START_TIME;

pub struct TestApp(pub StargazeApp);

impl TestApp {
    pub fn new() -> Self {
        Self(StargazeApp::default())
    }

    pub fn inner(&self) -> &StargazeApp {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut StargazeApp {
        &mut self.0
    }

    pub fn advance_time(&mut self, seconds: u64) {
        let mut block = self.0.block_info();
        block.time = block.time.plus_seconds(seconds);
        self.0.set_block(block);
    }

    pub fn set_genesis_time(&mut self) {
        let mut block = self.0.block_info();
        block.time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
        self.0.set_block(block);
    }

    pub fn get_balance(&self, address: &Addr, denom: &str) -> Option<u128> {
        self.0
            .wrap()
            .query_balance(address, denom)
            .ok()
            .map(|c| c.amount.u128())
    }

    pub fn store_code(&mut self, contract: Box<dyn Contract<StargazeMsgWrapper>>) -> u64 {
        self.0.store_code(contract)
    }

    pub fn store_tiles_code(&mut self) -> anyhow::Result<u64> {
        let contract = Box::new(ContractWrapper::new(
            tiles::contract::execute,
            tiles::contract::instantiate,
            tiles::contract::query,
        ));
        Ok(self.store_code(contract))
    }

    pub fn store_vending_factory_code(&mut self) -> anyhow::Result<u64> {
        let contract = Box::new(ContractWrapper::new(
            vending_factory::contract::execute,
            vending_factory::contract::instantiate,
            vending_factory::contract::query,
        ));
        Ok(self.store_code(contract))
    }

    pub fn store_vending_minter_code(&mut self) -> anyhow::Result<u64> {
        let contract = Box::new(
            ContractWrapper::new(
                vending_minter::contract::execute,
                vending_minter::contract::instantiate,
                vending_minter::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        Ok(self.store_code(contract))
    }

    pub fn instantiate_contract<T>(
        &mut self,
        code_id: u64,
        sender: Addr,
        msg: &T,
        funds: &[cosmwasm_std::Coin],
        label: &str,
        admin: Option<String>,
    ) -> anyhow::Result<Addr>
    where
        T: serde::Serialize,
    {
        self.0
            .instantiate_contract(code_id, sender, msg, funds, label, admin)
    }

    pub fn execute_contract<T>(
        &mut self,
        sender: Addr,
        contract: Addr,
        msg: &T,
        funds: &[cosmwasm_std::Coin],
    ) -> anyhow::Result<cw_multi_test::AppResponse>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        self.0.execute_contract(sender, contract, msg, funds)
    }

    pub fn clone_app(&self) -> TestApp {
        TestApp(StargazeApp::default())
    }
}

impl Default for TestApp {
    fn default() -> Self {
        let mut app = Self::new();
        app.set_genesis_time();
        app
    }
}
