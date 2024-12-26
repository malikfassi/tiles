use cosmwasm_std::{to_json_binary, Deps, Env, Response};
use sg_std::StargazeMsgWrapper;

use crate::contract::{
    contract::TilesContract,
    error::ContractError,
    msg::{CustomQueryMsg, QueryMsg},
    state::TILE_CONFIG,
};

pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract = TilesContract::default();

    let response = match msg {
        // Handle our custom queries
        QueryMsg::Custom(custom_msg) => match custom_msg {
            CustomQueryMsg::Config {} => {
                let config = TILE_CONFIG.load(deps.storage)?;
                to_json_binary(&config)?
            }
        },
        // Forward base queries to sg721-base
        QueryMsg::Base(base_msg) => contract.query(deps, env, base_msg)?,
    };

    Ok(Response::new().set_data(response))
}
