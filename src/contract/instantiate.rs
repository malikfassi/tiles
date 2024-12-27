use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, msg::InstantiateMsg, state::PRICE_SCALING},
    core::{pricing::PriceScaling, tile::Tile},
    defaults::constants::{CONTRACT_NAME, CONTRACT_VERSION},
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
    contract.instantiate(deps.branch(), env, info.clone(), msg)?;

    // Save default price scaling
    let price_scaling = PriceScaling::default();
    PRICE_SCALING.save(deps.storage, &price_scaling)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION))
}
