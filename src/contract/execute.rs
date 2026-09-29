use crate::contract::{
    error::ContractError,
    instantiate::TilesContract,
    msg::{ExecuteMsg, TileExecuteMsg},
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw721::traits::Cw721Execute;

/// Routes our extension messages and delegates every standard CW721 message to the base.
pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // CW721 0.22 routes contract-specific messages through `UpdateExtension`,
    // so the standard variants reach the base untouched.
    if let ExecuteMsg::UpdateExtension { msg } = msg {
        return match msg {
            TileExecuteMsg::SetPixelColor {
                token_id,
                current_metadata,
                updates,
            } => crate::contract::tiles::set_pixel_color::set_pixel_color(
                deps,
                env,
                info,
                token_id,
                current_metadata,
                updates,
            ),
            TileExecuteMsg::UpdatePriceScaling(new_scaling) => {
                crate::contract::tiles::update_price_scaling::update_price_scaling(
                    deps,
                    env,
                    info,
                    new_scaling,
                )
            }
        };
    }

    // Minting goes through our own handler, which seeds the tile with 100 blank pixels
    // before handing the call to the CW721 base.
    if let ExecuteMsg::Mint {
        token_id,
        owner,
        token_uri,
        ..
    } = msg
    {
        return crate::contract::tiles::mint::mint_handler(
            deps, env, info, token_id, owner, token_uri,
        );
    }

    let contract: TilesContract = TilesContract::default();
    Ok(contract.execute(deps, &env, &info, msg)?)
}
