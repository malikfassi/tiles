use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Response};
use sg_std::StargazeMsgWrapper;

use crate::contract::{error::ContractError, state::CONFIG};

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tile_royalty_payment_address: Option<String>,
    tile_royalty_fee_percent: Option<Decimal>,
    price_scaling: Option<crate::core::pricing::PriceScaling>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Check if sender is admin
    if info.sender != config.tile_admin_address {
        return Err(ContractError::Unauthorized {});
    }

    // Update fields if provided
    if let Some(new_address) = tile_royalty_payment_address {
        config.tile_royalty_payment_address = new_address;
    }

    if let Some(new_fee) = tile_royalty_fee_percent {
        config.tile_royalty_fee_percent = new_fee;
    }

    if let Some(new_price_scaling) = price_scaling {
        config.price_scaling = new_price_scaling;
    }

    // Validate entire config
    config.validate(deps.api).map_err(|e| ContractError::InvalidConfig(e.to_string()))?;

    // Save updated config
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
