use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{
        error::ContractError,
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    },
    core::tile::Tile,
};

pub type TilesContract<'a> = Sg721Contract<'a, Tile>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    crate::contract::instantiate::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    crate::contract::execute::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    crate::contract::query::query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    crate::contract::reply::reply(deps, env, msg)
}
