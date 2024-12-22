use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;
use sg721::InstantiateMsg as Sg721InstantiateMsg;
use sg721::CollectionInfo;

use crate::error::ContractError;
use crate::execute::{execute_set_pixel_color, execute_update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_config, query_tile_state};
use crate::state::CONFIG;
use crate::types::{Config, Extension};
use crate::utils::price;

pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Validate configuration
    price::validate_price_scaling(&msg.price_scaling)?;

    // Convert collection info
    let collection_info = CollectionInfo {
        creator: msg.collection_info.creator,
        description: msg.collection_info.description,
        image: msg.collection_info.image,
        external_link: msg.collection_info.external_link,
        royalty_info: msg.collection_info.royalty_info,
        explicit_content: Some(false),
        start_trading_time: None,
    };

    // Initialize base contract
    let contract: Sg721Contract<Extension> = Sg721Contract::default();
    let sg721_msg = Sg721InstantiateMsg {
        name: msg.name.clone(),
        symbol: msg.symbol,
        minter: msg.minter.clone(),
        collection_info: collection_info.clone(),
    };
    contract.instantiate(deps.branch(), env.clone(), info.clone(), sg721_msg)?;

    // Save our custom config
    CONFIG.save(
        deps.storage,
        &Config {
            minter: deps.api.addr_validate(&msg.minter)?,
            dev_address: deps.api.addr_validate(&msg.dev_address)?,
            dev_fee_percent: msg.dev_fee_percent,
            base_price: msg.base_price,
            price_scaling: msg.price_scaling,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract_name", msg.name)
        .add_attribute("contract_version", env.contract.address.to_string()))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Sg721(base_msg) => {
            let contract: Sg721Contract<Extension> = Sg721Contract::default();
            let res = contract.execute(deps, env, info, base_msg)?;
            Ok(res)
        }
        ExecuteMsg::SetPixelColor(msg) => execute_set_pixel_color(deps, env, info, msg),
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Sg721(base_msg) => {
            let contract: Sg721Contract<Extension> = Sg721Contract::default();
            contract.query(deps, env, base_msg)
        }
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::TileState { token_id } => to_json_binary(&query_tile_state(deps, token_id)?),
    }
}
