use crate::{
    contract::{
        error::ContractError,
        state::{CONFIG, PRICE_SCALING},
    },
    core::pricing::PriceScaling,
    events::{EventData, PriceScalingUpdateEventData},
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Replaces the duration-based price grid.
///
/// Restricted to the collection payment address configured at instantiation, which is
/// the address that also holds the CW721 collection royalties.
pub fn update_price_scaling(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_scaling: Box<PriceScaling>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.collection_payment_address {
        return Err(ContractError::Unauthorized {});
    }

    new_scaling
        .validate()
        .map_err(|_| ContractError::InvalidPriceScaling {})?;

    PRICE_SCALING.save(deps.storage, &new_scaling)?;

    let event = PriceScalingUpdateEventData {
        hour_1_price: new_scaling.hour_1_price.u128(),
        hour_12_price: new_scaling.hour_12_price.u128(),
        hour_24_price: new_scaling.hour_24_price.u128(),
        quadratic_base: new_scaling.quadratic_base.u128(),
    }
    .into_event();

    Ok(Response::new().add_event(event))
}
