use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, Response};
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

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    crate::contract::instantiate::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    crate::contract::execute::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    crate::contract::query::query(deps, env, msg)
}
