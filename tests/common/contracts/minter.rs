use anyhow::Result;
use cosmwasm_std::{Addr, coins};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::MINT_PRICE;
use vending_minter::msg::ExecuteMsg;

use crate::common::app::TestApp;

pub struct MinterContract {
    pub contract_addr: Addr,
}

impl MinterContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    pub fn store_code(app: &mut TestApp) -> Result<u64> {
        let contract = Box::new(
            ContractWrapper::new(
                vending_minter::contract::execute,
                vending_minter::contract::instantiate,
                vending_minter::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        Ok(app.store_code(contract))
    }

    /// Mints a new token for the given buyer
    /// Returns the AppResponse which can be used to extract the token_id
    pub fn mint(&self, app: &mut TestApp, buyer: &Addr) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            buyer.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Mint {},
            &coins(MINT_PRICE.into(), NATIVE_DENOM),
        )
    }
}
