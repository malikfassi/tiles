use crate::contract::{
    instantiate::TilesContract,
    msg::{QueryMsg, TileQueryMsg},
    state::{CONFIG, PRICE_SCALING},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError, StdResult};
use cw721::traits::Cw721Query;

/// Routes our custom queries and delegates every standard CW721 query to the base.
pub fn query_handler(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // Custom queries travel through the extension slot of the CW721 query enum.
    if let QueryMsg::Extension { msg } = msg {
        return query_custom(deps, msg);
    }

    let contract: TilesContract = TilesContract::default();
    contract
        .query(deps, &env, msg)
        .map_err(|e| StdError::generic_err(e.to_string()))
}

fn query_custom(deps: Deps, msg: TileQueryMsg) -> StdResult<Binary> {
    match msg {
        TileQueryMsg::PriceScaling {} => to_json_binary(&PRICE_SCALING.load(deps.storage)?),
        TileQueryMsg::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
        TileQueryMsg::TilePixels { token_id } => {
            let contract: TilesContract = TilesContract::default();
            let token = contract
                .config
                .nft_info
                .may_load(deps.storage, &token_id)?
                .ok_or_else(|| StdError::not_found(format!("tile {}", token_id)))?;
            to_json_binary(&token.extension.metadata)
        }
    }
}
