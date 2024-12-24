use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use sg_std::StargazeMsgWrapper;
use cw2::set_contract_version;
use sg721_base::{ContractError, Sg721Contract};
use crate::core::tile::Tile;

use crate::contract::{
    execute::execute_handler,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::handle_query,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tiles";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Contract<'a> {
    pub base: Sg721Contract<'a, Tile>,
}

impl<'a> Default for Contract<'a> {
    fn default() -> Self {
        Self {
            base: Sg721Contract::<Tile>::default(),
        }
    }
}

impl<'a> Contract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<StargazeMsgWrapper>, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        
        // Extract minter before moving msg
        let minter = deps.api.addr_validate(&msg.minter)?;
        
        // Set up default config
        let config = crate::contract::state::Config {
            admin: info.sender.clone(),
            minter: minter,
            dev_address: info.sender.clone(),
            dev_fee_percent: 5, // 5% default
            base_price: 1_000_000, // 1 STARS default
            price_scaling: crate::contract::state::PriceScaling {
                hour_1_price: 100_000_000,  // 100 STARS
                hour_12_price: 200_000_000, // 200 STARS
                hour_24_price: 300_000_000, // 300 STARS
                quadratic_base: 400_000_000, // 400 STARS
            },
        };
        crate::contract::state::CONFIG.save(deps.storage, &config)?;

        // Initialize sg721 contract
        let res = self.base.instantiate(deps, env.clone(), info, msg)?;
        
        Ok(Response::new()
            .add_attributes(res.attributes)
            .add_events(res.events)
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response<StargazeMsgWrapper>, ContractError> {
        execute_handler(self, deps, env, info, msg)
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        handle_query(self, deps, env, msg)
    }
}

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use cosmwasm_std::entry_point;

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<StargazeMsgWrapper>, ContractError> {
        Contract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response<StargazeMsgWrapper>, ContractError> {
        Contract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Contract::default().query(deps, env, msg)
    }
}
