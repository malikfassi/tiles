use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg_std::StargazeMsgWrapper;

use crate::contract::{
    contract::TilesContract, error::ContractError, msg::InstantiateMsg, state::TILE_CONFIG,
};
use crate::core::Config;

pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract = TilesContract::default();

    // Initialize base contract
    contract.instantiate(deps.branch(), env.clone(), info.clone(), msg)?;

    // Initialize our config
    let config = Config {
        dev_address: info.sender.clone(),
        tile_royalty_payment_address: "".to_string(),
        tile_royalty_fee_percent: Default::default(),
        price_scaling: Default::default(),
    };
    TILE_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}
