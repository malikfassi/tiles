use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{
        error::ContractError,
        execute::execute_handler,
        instantiate::instantiate_handler,
        query::query_handler,
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
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    query_handler(deps, env, msg)
}
