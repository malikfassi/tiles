use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw721_base::Extension;

use sg_std::StargazeMsgWrapper;
use sg721_base::Sg721Contract;

use crate::contract::{
    error::ContractError,
    execute::execute_handler,
    instantiate::instantiate_handler,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::query_handler,
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query_handler(deps, env, msg)
}
