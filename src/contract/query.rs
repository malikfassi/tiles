use cosmwasm_std::{to_json_binary, Binary, Deps, Env};
use sg721_base::Sg721Contract;

use crate::{
    contract::{error::ContractError, msg::QueryMsg, state::CONFIG},
    core::tile::Tile,
};

pub fn query_handler(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();

    match msg {
        QueryMsg::Config {} => {
            let config = CONFIG.load(deps.storage)?;
            Ok(to_json_binary(&config)?)
        }
        QueryMsg::Base(base_msg) => {
            let res = contract.query(deps, env, base_msg)?;
            Ok(res)
        }
    }
}
