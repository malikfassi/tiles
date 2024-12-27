use cosmwasm_std::{DepsMut, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::contract::{error::ContractError, state::PRICE_SCALING};
use crate::core::{pricing::PriceScaling, tile::Tile};

pub fn update_price_scaling(
    deps: DepsMut,
    info: MessageInfo,
    new_price_scaling: PriceScaling,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Get collection info through the contract query
    let contract: Sg721Contract<Tile> = Sg721Contract::default();
    let collection_info = contract.query_collection_info(deps.as_ref())?;

    // Only royalty payment address can update prices
    if let Some(royalty_info) = collection_info.royalty_info {
        if info.sender != deps.api.addr_validate(&royalty_info.payment_address)? {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }

    // Validate new price scaling
    new_price_scaling
        .validate()
        .map_err(|e| ContractError::InvalidConfig(e.to_string()))?;

    // Save updated price scaling
    PRICE_SCALING.save(deps.storage, &new_price_scaling)?;

    Ok(Response::new().add_attribute("action", "update_price_scaling"))
}
