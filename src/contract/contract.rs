use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, to_json_binary};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{
        error::ContractError,
        execute::execute_handler,
        instantiate::instantiate_handler,
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
        state::CONFIG,
    },
    core::tile::Tile,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    instantiate_handler(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    execute_handler(deps, env, info, msg)
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();
    
    match msg {
        QueryMsg::Config {} => {
            let config = CONFIG.load(deps.storage)?;
            Ok(to_json_binary(&config)?)
        }
        QueryMsg::Base(base_msg) => {
            Ok(contract.query(deps, env, base_msg)?)
        }
    }
}
