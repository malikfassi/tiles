use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::{msg::NftParams, Sg721Contract};
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::error::ContractError,
    core::tile::{metadata::TileMetadata, Tile},
};

pub fn mint_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
    extension: Option<Tile>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();

    // Set default metadata if none provided
    let extension = extension.unwrap_or_else(|| Tile {
        tile_hash: TileMetadata::default().hash(),
    });

    // Forward to base contract with our extension
    Ok(contract.mint(
        deps,
        env,
        info,
        NftParams::NftData {
            token_id,
            owner,
            token_uri,
            extension,
        },
    )?)
}
