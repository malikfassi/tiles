use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError, StdResult};

use crate::{
    contract::{
        contract::Contract,
        msg::{QueryMsg, Extension},
        state::CONFIG,
    },
    core::tile::metadata::TileMetadata,
};

pub fn handle_query(contract: &Contract, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Extension(extension) => match extension {
            Extension::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
            Extension::PixelState { token_id, position } => {
                // Load tile metadata from storage
                let metadata_bytes = deps.storage
                    .get(format!("tile_metadata:{}", token_id).as_bytes())
                    .ok_or_else(|| StdError::not_found("Tile metadata"))?;
                
                let metadata: TileMetadata = cosmwasm_std::from_json(&metadata_bytes)?;
                
                // Get pixel data
                let pixel = metadata
                    .get_pixel(position)
                    .map_err(|e| StdError::generic_err(e.to_string()))?;
                    
                to_json_binary(&pixel)
            }
        },
        QueryMsg::Sg721(msg) => contract.base.query(deps, env, msg),
    }
}
