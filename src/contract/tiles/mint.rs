use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw721_base::state::TokenInfo;
use sg_std::StargazeMsgWrapper;

use crate::contract::{contract::TilesContract, error::ContractError};
use crate::core::tile::Tile;

pub fn mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract = TilesContract::default();

    let token = TokenInfo {
        owner: deps.api.addr_validate(&owner)?,
        approvals: vec![],
        token_uri,
        extension: Tile::default(),
    };

    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("token_id", token_id)
        .add_attribute("owner", owner))
}
