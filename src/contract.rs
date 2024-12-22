use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, StdResult,
    Uint128,
};

use cw2::set_contract_version;
use sg721::InstantiateMsg as Sg721InstantiateMsg;
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

pub type ContractResponse = cosmwasm_std::Response<StargazeMsgWrapper>;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, Extension, PriceScaling, CONFIG},
};

const CONTRACT_NAME: &str = "crates.io:tiles";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Default values for config
const DEFAULT_DEV_FEE_PERCENT: u64 = 5; // 5%
const DEFAULT_BASE_PRICE: u128 = 100_000; // 0.1 STARS

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<ContractResponse, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Initialize sg721-base contract
    let sg721_base = Sg721Contract::<Extension>::default();
    let sg721_msg = Sg721InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: msg.minter.clone(),
        collection_info: msg.collection_info.clone(),
    };
    sg721_base.instantiate(deps.branch(), env, info.clone(), sg721_msg)?;

    // Set default config values
    let base_price = Uint128::new(DEFAULT_BASE_PRICE);
    let config = Config {
        admin: info.sender.clone(),
        minter: deps.api.addr_validate(&msg.minter)?,
        collection_info: msg.collection_info,
        dev_address: info.sender.clone(), // Default to sender
        dev_fee_percent: Decimal::percent(DEFAULT_DEV_FEE_PERCENT),
        base_price,
        price_scaling: Some(PriceScaling {
            hour_1_price: base_price,
            hour_12_price: base_price.checked_mul(Uint128::from(2u128)).unwrap(),
            hour_24_price: base_price.checked_mul(Uint128::from(3u128)).unwrap(),
            quadratic_base: base_price.checked_mul(Uint128::from(4u128)).unwrap(),
        }),
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(ContractResponse::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender)
        .add_attribute("minter", msg.minter))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<ContractResponse, ContractError> {
    crate::execute::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Sg721Base(base_msg) => {
            let contract = Sg721Contract::<Extension>::default();
            contract.query(deps, env, base_msg)
        }
        QueryMsg::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
    }
}
