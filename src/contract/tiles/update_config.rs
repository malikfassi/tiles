use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Decimal};
use sg_std::StargazeMsgWrapper;

use crate::contract::{
    error::ContractError,
    state::TILE_CONFIG,
};
use crate::core::pricing::PriceScaling;

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tile_royalty_payment_address: Option<String>,
    tile_royalty_fee_percent: Option<Decimal>,
    price_scaling: Option<PriceScaling>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let mut config = TILE_CONFIG.load(deps.storage)?;

    // Only contract owner can update config
    if info.sender != config.dev_address {
        return Err(ContractError::Unauthorized {});
    }

    // Update config fields if provided
    if let Some(addr) = tile_royalty_payment_address {
        config.tile_royalty_payment_address = deps.api.addr_validate(&addr)?.to_string();
    }

    if let Some(fee) = tile_royalty_fee_percent {
        config.tile_royalty_fee_percent = fee;
    }

    if let Some(scaling) = price_scaling {
        config.price_scaling = scaling;
    }

    // Save updated config
    TILE_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
} 