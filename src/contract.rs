use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, to_json_binary, CosmosMsg};
use sg721_base::Sg721Contract;
use sg_std::Response;

use crate::error::ContractError;
use crate::execute::{execute_set_pixel_color, execute_update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Extension, UpdateConfigMsg};
use crate::query::{query_config, query_tile_state};
use crate::state::{Config, CONFIG};

pub struct TilesContract<'a> {
    pub sg721_base: Sg721Contract<'a, Extension>,
}

impl<'a> Default for TilesContract<'a> {
    fn default() -> Self {
        Self {
            sg721_base: Sg721Contract::default(),
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // First instantiate base sg721 contract
    let sg721_msg = msg.clone().into();
    TilesContract::default().sg721_base.instantiate(deps.branch(), env.clone(), info.clone(), sg721_msg)?;

    // Then initialize our extension
    let config = Config {
        admin: deps.api.addr_validate(&msg.minter)?,
        minter: deps.api.addr_validate(&msg.minter)?,
        collection_info: msg.collection_info,
        dev_address: deps.api.addr_validate(&msg.dev_address)?,
        dev_fee_percent: msg.dev_fee_percent,
        base_price: msg.base_price,
        price_scaling: Some(msg.price_scaling),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", config.admin)
        .add_attribute("minter", msg.minter)
        .add_attribute("dev_address", msg.dev_address)
        .add_attribute("dev_fee_percent", msg.dev_fee_percent.to_string())
        .add_attribute("base_price", msg.base_price.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPixelColor(msg) => execute_set_pixel_color(deps, env, info, msg),
        ExecuteMsg::UpdateConfig {
            dev_address,
            dev_fee_percent,
            base_price,
            price_scaling,
        } => execute_update_config(
            deps,
            env,
            info,
            UpdateConfigMsg {
                dev_address,
                dev_fee_percent,
                base_price,
                price_scaling,
            },
        ),
        ExecuteMsg::Base(base_msg) => {
            let res = TilesContract::default().sg721_base.execute(deps, env, info, base_msg)
                .map_err(ContractError::Sg721Error)?;
            
            // Convert Stargaze response to standard response
            let mut response = Response::new()
                .add_attributes(res.attributes)
                .add_events(res.events);
            
            // Convert messages
            for msg in res.messages {
                response = response.add_message(msg.msg);
            }
            
            Ok(response)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::TileState { token_id } => to_json_binary(&query_tile_state(deps, token_id)?),
        QueryMsg::Base(base_msg) => {
            TilesContract::default().sg721_base.query(deps, env, base_msg)
        }
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
            price_scaling: crate::state::PriceScaling {
                hour_1_price: Uint128::new(100),
                hour_12_price: Uint128::new(200),
                hour_24_price: Uint128::new(300),
                quadratic_base: Uint128::new(400),
            },
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!("creator", config.admin);
        assert_eq!("minter", config.minter);
        assert_eq!("dev", config.dev_address);
        assert_eq!(Decimal::percent(5), config.dev_fee_percent);
        assert_eq!(Uint128::new(100), config.base_price);
    }
} 