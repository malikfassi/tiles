use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg_std::StargazeMsgWrapper;
use sg721_base::Sg721Contract;

use crate::{
    contract::{
        error::ContractError,
        msg::{ExecuteMsg, TileExecuteMsg},
        tiles::{set_pixel_color::set_pixel_color, update_config::update_config, mint::mint_handler},
    },
    core::tile::Tile,
};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();
    
    match msg {
        ExecuteMsg::Extension { msg } => match msg {
            TileExecuteMsg::SetPixelColor { token_id, current_metadata, updates } => 
                set_pixel_color(deps, env, info, token_id, current_metadata, updates),
            TileExecuteMsg::UpdateConfig { tile_royalty_payment_address, tile_royalty_fee_percent, price_scaling } => 
                update_config(deps, env, info, tile_royalty_payment_address, tile_royalty_fee_percent, price_scaling),
        },
        ExecuteMsg::Mint { token_id, owner, token_uri, extension } => {
            mint_handler(deps, env, info, token_id, owner, token_uri, Some(extension))
        },
        ExecuteMsg::UpdateOwnership(action) => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::UpdateOwnership(action))?)
        },
        // Forward all other messages to base contract
        ExecuteMsg::TransferNft { recipient, token_id } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::TransferNft { recipient, token_id })?)
        },
        ExecuteMsg::SendNft { contract: contract_addr, token_id, msg } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::SendNft { contract: contract_addr, token_id, msg })?)
        },
        ExecuteMsg::Approve { spender, token_id, expires } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::Approve { spender, token_id, expires })?)
        },
        ExecuteMsg::Revoke { spender, token_id } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::Revoke { spender, token_id })?)
        },
        ExecuteMsg::ApproveAll { operator, expires } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::ApproveAll { operator, expires })?)
        },
        ExecuteMsg::RevokeAll { operator } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::RevokeAll { operator })?)
        },
        ExecuteMsg::Burn { token_id } => {
            Ok(contract.execute(deps, env, info, sg721::ExecuteMsg::Burn { token_id })?)
        },
    }
}
