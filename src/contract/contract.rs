use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::contract::{
    error::ContractError,
    execute::execute_handler,
    instantiate::instantiate_handler,
    migrate::migrate_handler,
    msg::{ExecuteMsg, MigrateMsg, QueryMsg},
    query::query_handler,
};

/// Instantiate message: the CW721 instantiate message, which already carries
/// `collection_info_extension` (description, image, royalties) as Stargaze 2.0 documents it.
pub use crate::contract::instantiate::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    instantiate_handler(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    execute_handler(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query_handler(deps, env, msg)
}

/// Migrates the stored state to the layout of this code.
///
/// See `migrate_handler` for the rules applied: the contract name must match, migrating
/// from the current version is a no-op, and any other source version is refused rather
/// than guessed at.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_handler(deps, env, msg)
}
