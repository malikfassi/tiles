use cosmwasm_std::{DepsMut, Env, Reply, Response};
use sg_std::StargazeMsgWrapper;

use crate::contract::error::ContractError;

pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // For now, just return an empty response since we don't need to handle any replies
    Ok(Response::new())
} 