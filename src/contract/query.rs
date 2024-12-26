use cosmwasm_std::{to_json_binary, Binary, Deps, Env};

use crate::contract::{
    contract::TilesContract,
    error::ContractError,
    msg::{CustomQueryMsg, QueryMsg},
    state::TILE_CONFIG,
};

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let contract = TilesContract::default();

    match msg {
        // Handle our custom queries
        QueryMsg::Custom(custom_msg) => match custom_msg {
            CustomQueryMsg::Config {} => {
                let config = TILE_CONFIG.load(deps.storage)?;
                Ok(to_json_binary(&config)?)
            }
        },
        // Forward base queries to sg721-base
        QueryMsg::Base(base_msg) => Ok(contract.query(deps, env, base_msg)?),
    }
}
