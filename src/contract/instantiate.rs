use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use serde_json;
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, msg::InstantiateMsg, state::PRICE_SCALING},
    core::{pricing::PriceScaling, tile::Tile},
    defaults::constants::{CONTRACT_NAME, CONTRACT_VERSION},
    events::{EventData, InstantiateConfigEventData},
};

pub fn instantiate_handler(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Initialize base contract
    let contract = Sg721Contract::<Tile>::default();
    contract.instantiate(deps.branch(), env.clone(), info.clone(), msg.clone())?;

    // Save default price scaling
    let price_scaling = PriceScaling::default();
    PRICE_SCALING.save(deps.storage, &price_scaling)?;

    // Create instantiate event with config
    let config_event = InstantiateConfigEventData {
        collection_info: serde_json::to_string(&msg.collection_info).unwrap_or_default(),
        minter: msg.minter,
        price_scaling: serde_json::to_string(&price_scaling).unwrap_or_default(),
        time: env.block.time.to_string(),
    }
    .into_event();

    Ok(Response::new()
        .add_event(config_event)
        .add_attribute("method", "instantiate"))
}
