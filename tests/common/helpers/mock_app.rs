use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{Contract, Executor};
use sg_multi_test::StargazeApp;
use sg_std::{Response as SgResponse, StargazeMsgWrapper};

pub struct MockApp {
    pub app: StargazeApp,
}

impl MockApp {
    pub fn new() -> Self {
        let app = StargazeApp::default();
        Self { app }
    }

    pub fn sender() -> Addr {
        Addr::unchecked("sender")
    }

    pub fn store_code(&mut self, contract: Box<dyn Contract<StargazeMsgWrapper>>) -> u64 {
        self.app.store_code(contract)
    }

    pub fn instantiate_contract<T>(
        &mut self,
        code_id: u64,
        msg: T,
        admin: Option<&str>,
        label: &str,
    ) -> StdResult<Addr>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        let sender = Self::sender();
        self.app
            .instantiate_contract(
                code_id,
                sender,
                &msg,
                &[],
                label,
                admin.map(|a| a.to_string()),
            )
            .map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))
    }

    pub fn execute_contract<T>(
        &mut self,
        sender: &str,
        contract: &Addr,
        msg: T,
    ) -> Result<SgResponse, tiles::contract::error::ContractError>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        let res = self
            .app
            .execute_contract(Addr::unchecked(sender), contract.clone(), &msg, &[])
            .map_err(|e| {
                tiles::contract::error::ContractError::Std(cosmwasm_std::StdError::generic_err(
                    e.to_string(),
                ))
            })?;

        Ok(SgResponse::new().add_events(res.events))
    }

    pub fn query_wasm_smart<T, U>(&self, contract: &Addr, msg: &T) -> StdResult<U>
    where
        T: serde::Serialize + std::fmt::Debug,
        U: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        self.app
            .wrap()
            .query_wasm_smart(contract, msg)
            .map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))
    }
}
