use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{
        error::ContractError,
        msg::InstantiateMsg,
        state::CONFIG,
    },
    core::tile::Tile,
};

pub fn instantiate_handler(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();
    
    // Save default config first
    CONFIG.save(deps.storage, &Default::default())?;

    // Then initialize base contract
    contract.instantiate(deps, env, info.clone(), msg)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "tiles"))
}
