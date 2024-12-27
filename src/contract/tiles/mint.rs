use cosmwasm_std::{to_json_binary, DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, msg::Sg721ExecuteMsg},
    core::tile::{metadata::TileMetadata, Tile},
};

pub fn mint_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();

    // Generate our own extension
    let extension = Tile {
        tile_hash: TileMetadata::default().hash(),
    };

    // Create mint message with our Tile extension
    let mint_msg = Sg721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: token_uri.clone(),
        extension: extension.clone(),
    };

    // Forward to base contract with our extension
    Ok(contract.execute(deps, env, info, mint_msg)?)
}
