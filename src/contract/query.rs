use cosmwasm_std::{to_json_binary, Binary, Deps, Env};
use sg721_base::Sg721Contract;

use crate::{
    contract::{error::ContractError, msg::QueryMsg, state::PRICE_SCALING},
    core::tile::Tile,
};

pub fn query_handler(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();

    match msg {
        QueryMsg::PriceScaling {} => {
            let price_scaling = PRICE_SCALING.load(deps.storage)?;
            Ok(to_json_binary(&price_scaling)?)
        }
        QueryMsg::Base(base_msg) => {
            let res = contract.query(deps, env, base_msg)?;
            Ok(res)
        }
    }
}
