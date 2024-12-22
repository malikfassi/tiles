use cosmwasm_std::{Deps, StdResult};

use crate::state::{Config, CONFIG, TILE_STATES, TileState};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_tile_state(deps: Deps, token_id: String) -> StdResult<TileState> {
    TILE_STATES.load(deps.storage, &token_id)
} 