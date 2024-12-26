use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Response};
use sg_std::StargazeMsgWrapper;

use crate::contract::{error::ContractError, state::CONFIG};
use crate::core::pricing::PriceScaling;

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tile_royalty_payment_address: Option<String>,
    tile_royalty_fee_percent: Option<Decimal>,
    price_scaling: Option<PriceScaling>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only owner can update config
    if info.sender != config.tile_admin_address {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(tile_royalty_payment_address) = tile_royalty_payment_address {
        config.tile_royalty_payment_address = tile_royalty_payment_address;
    }

    if let Some(tile_royalty_fee_percent) = tile_royalty_fee_percent {
        config.tile_royalty_fee_percent = tile_royalty_fee_percent;
    }

    if let Some(price_scaling) = price_scaling {
        config.price_scaling = price_scaling;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("sender", info.sender))
}
