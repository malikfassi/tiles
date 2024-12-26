use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg_std::StargazeMsgWrapper;

use crate::contract::{
    contract::TilesContract,
    error::ContractError,
    msg::{CustomExecuteMsg, ExecuteMsg},
    tiles::{self, set_pixel_color, update_config},
};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract = TilesContract::default();

    match msg {
        ExecuteMsg::Custom(msg) => match msg {
            CustomExecuteMsg::SetPixelColor {
                token_id,
                current_metadata,
                updates,
            } => set_pixel_color::set_pixel_color(
                deps,
                env,
                info,
                token_id,
                current_metadata,
                updates,
            ),
            CustomExecuteMsg::UpdateConfig {
                tile_royalty_payment_address,
                tile_royalty_fee_percent,
                price_scaling,
            } => update_config::update_config(
                deps,
                env,
                info,
                tile_royalty_payment_address,
                tile_royalty_fee_percent,
                price_scaling,
            ),
        },
        ExecuteMsg::Base(base_msg) => match base_msg {
            sg721::ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                ..
            } => tiles::mint::mint(deps, env, info, token_id, owner, token_uri),
            _ => contract
                .execute(deps, env, info, base_msg)
                .map_err(ContractError::Base),
        },
    }
}
