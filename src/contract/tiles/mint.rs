use crate::{
    contract::{error::ContractError, instantiate::TilesContract},
    core::tile::{metadata::TileMetadata, Tile},
    events::{EventData, MintMetadataEventData},
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw721::traits::Cw721Execute;

/// Mints a tile to its first owner.
///
/// Restricted by the CW721 base to the configured minter. The tile starts with 100 blank
/// pixels; colouring them is a separate, paid operation (`SetPixelColor`).
pub fn mint_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
) -> Result<Response, ContractError> {
    let metadata = TileMetadata::default();
    let extension = Tile {
        tile_hash: metadata.hash(),
        metadata: metadata.clone(),
    };

    let owner_addr = deps.api.addr_validate(&owner)?;

    let metadata_event = MintMetadataEventData {
        token_id: token_id.clone(),
        owner: owner_addr,
        new_pixels: metadata.pixels.clone(),
        tile_hash: extension.tile_hash.clone(),
    }
    .into_event();

    let contract: TilesContract = TilesContract::default();
    let response = contract.mint(deps, &env, &info, token_id, owner, token_uri, extension)?;
    Ok(response.add_event(metadata_event))
}
