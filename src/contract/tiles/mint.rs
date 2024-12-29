use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, msg::Sg721ExecuteMsg},
    core::tile::{metadata::TileMetadata, Tile},
    events::{EventData, MintMetadataEventData},
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

    // Generate initial metadata
    let metadata = TileMetadata::default();

    // Generate our own extension
    let extension = Tile {
        tile_hash: metadata.hash(),
    };

    // Create mint message with our Tile extension
    let mint_msg = Sg721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: token_uri.clone(),
        extension: extension.clone(),
    };

    // Create mint metadata event
    let metadata_event = MintMetadataEventData {
        token_id,
        owner: deps.api.addr_validate(&owner)?,
        new_pixels: metadata.pixels,
        tile_hash: extension.tile_hash,
    }
    .into_event();

    // Forward to base contract with our extension and add our event
    let response = contract.execute(deps, env, info, mint_msg)?;
    Ok(response.add_event(metadata_event))
}
