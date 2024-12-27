use cosmwasm_std::{to_json_binary, DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{
        error::ContractError,
        msg::{ExecuteMsg, Sg721ExecuteMsg, TileExecuteMsg},
        tiles::{
            mint::mint_handler, set_pixel_color::set_pixel_color,
            update_price_scaling::update_price_scaling,
        },
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
            TileExecuteMsg::SetPixelColor {
                token_id,
                current_metadata,
                updates,
            } => set_pixel_color(deps, env, info, token_id, current_metadata, updates),
            TileExecuteMsg::UpdatePriceScaling(new_scaling) => {
                update_price_scaling(deps, env, info, new_scaling)
            }
        },
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension: _,
        } => mint_handler(deps, env, info, token_id, owner, token_uri),
        ExecuteMsg::UpdateOwnership(action) => {
            let base_msg = Sg721ExecuteMsg::UpdateOwnership(action);
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => {
            let base_msg = Sg721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::SendNft {
            contract: contract_addr,
            token_id,
            msg,
        } => {
            let base_msg = Sg721ExecuteMsg::SendNft {
                contract: contract_addr,
                token_id,
                msg,
            };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => {
            let base_msg = Sg721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::Revoke { spender, token_id } => {
            let base_msg = Sg721ExecuteMsg::Revoke { spender, token_id };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::ApproveAll { operator, expires } => {
            let base_msg = Sg721ExecuteMsg::ApproveAll { operator, expires };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::RevokeAll { operator } => {
            let base_msg = Sg721ExecuteMsg::RevokeAll { operator };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::Burn { token_id } => {
            let base_msg = Sg721ExecuteMsg::Burn { token_id };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::UpdateCollectionInfo { collection_info } => {
            let base_msg = Sg721ExecuteMsg::UpdateCollectionInfo { collection_info };
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::UpdateStartTradingTime(time) => {
            let base_msg = Sg721ExecuteMsg::UpdateStartTradingTime(time);
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
        ExecuteMsg::FreezeCollectionInfo => {
            let base_msg = Sg721ExecuteMsg::FreezeCollectionInfo;
            Ok(contract.execute(deps, env, info, base_msg)?)
        }
    }
}
