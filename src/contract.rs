#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    StdResult, WasmMsg,
};
use cw2::set_contract_version;
use sg_std::StargazeMsgWrapper;
use sg721_base::Sg721Contract;

pub type Response = cosmwasm_std::Response<StargazeMsgWrapper>;
pub type SubMsg = cosmwasm_std::SubMsg<StargazeMsgWrapper>;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Extension};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tiles";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admin: info.sender.clone(),
        minter: deps.api.addr_validate(&msg.minter)?,
        collection_info: msg.collection_info,
        dev_address: deps.api.addr_validate(&msg.dev_address)?,
        dev_fee_percent: msg.dev_fee_percent,
        base_price: msg.base_price,
        price_scaling: msg.price_scaling,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender)
        .add_attribute("minter", msg.minter))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Sg721Base(msg) => {
            let config = CONFIG.load(deps.storage)?;
            if info.sender != config.minter {
                return Err(ContractError::Unauthorized {});
            }
            let msg = WasmMsg::Execute {
                contract_addr: config.minter.to_string(),
                msg: to_json_binary(&msg)?,
                funds: vec![],
            };
            Ok(Response::new().add_message(msg))
        }
        ExecuteMsg::SetPixelColor(msg) => crate::execute::execute_set_pixel_color(deps, env, info, msg),
        ExecuteMsg::UpdateConfig(msg) => crate::execute::execute_update_config(deps, env, info, msg),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Sg721Base(base_msg) => {
            let contract: Sg721Contract<Extension> = Sg721Contract::default();
            contract.query(deps, env, base_msg)
        }
        QueryMsg::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Decimal, Uint128};
    use sg721::CollectionInfo;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            name: "Tiles".to_string(),
            symbol: "TILE".to_string(),
            minter: "minter".to_string(),
            collection_info: CollectionInfo {
                creator: "creator".to_string(),
                description: "Tile NFTs".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: Some("https://example.com/external.html".to_string()),
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
            dev_address: "dev".to_string(),
            dev_fee_percent: Decimal::percent(5),
            base_price: Uint128::new(100),
            price_scaling: Some(crate::state::PriceScaling {
                hour_1_price: Uint128::new(100),
                hour_12_price: Uint128::new(200),
                hour_24_price: Uint128::new(300),
                quadratic_base: Uint128::new(400),
            }),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let config: Config = from_json(res).unwrap();
        assert_eq!("creator", config.admin);
        assert_eq!("minter", config.minter);
        assert_eq!("dev", config.dev_address);
        assert_eq!(Decimal::percent(5), config.dev_fee_percent);
        assert_eq!(Uint128::new(100), config.base_price);
    }
} 