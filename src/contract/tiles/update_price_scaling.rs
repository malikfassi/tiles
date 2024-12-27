use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, state::PRICE_SCALING},
    core::{pricing::PriceScaling, tile::Tile},
    events::{EventData, PriceScalingUpdateEventData},
};

pub fn update_price_scaling(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_scaling: PriceScaling,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Get collection info from contract
    let contract = Sg721Contract::<Tile>::default();
    let collection_info = contract.collection_info.load(deps.storage)?;

    // Only royalty payment address can update prices
    if let Some(royalty_info) = collection_info.royalty_info {
        if info.sender != royalty_info.payment_address {
            return Err(ContractError::Unauthorized {
                sender: info.sender.to_string(),
            });
        }
    } else {
        return Err(ContractError::MissingRoyaltyInfo {});
    }

    // Validate new price scaling
    match new_scaling.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ContractError::InvalidPixelUpdate { 
                reason: e.to_string() 
            });
        }
    }

    // Save new price scaling
    PRICE_SCALING.save(deps.storage, &new_scaling)?;

    // Create event
    let event = PriceScalingUpdateEventData {
        hour_1_price: new_scaling.hour_1_price.u128(),
        hour_12_price: new_scaling.hour_12_price.u128(),
        hour_24_price: new_scaling.hour_24_price.u128(),
        quadratic_base: new_scaling.quadratic_base.u128(),
    }.into_event();

    Ok(Response::new().add_event(event))
}
