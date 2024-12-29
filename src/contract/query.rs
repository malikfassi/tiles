use crate::core::tile::Tile;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};
use cw721_base::Extension;
use serde::{Deserialize, Serialize};
use sg721_base::{msg::QueryMsg as Sg721QueryMsg, Sg721Contract};

use crate::contract::{msg::QueryMsg, state::PRICE_SCALING};

pub fn query_handler(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PriceScaling {} => to_json_binary(&PRICE_SCALING.load(deps.storage)?),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => {
            let base_msg = Sg721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            };
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired,
        } => {
            let base_msg = Sg721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            };
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => {
            let base_msg = Sg721QueryMsg::Approvals {
                token_id,
                include_expired,
            };
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit,
        } => {
            let base_msg = Sg721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            };
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::NumTokens {} => {
            let base_msg = Sg721QueryMsg::NumTokens {};
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::ContractInfo {} => {
            let base_msg = Sg721QueryMsg::ContractInfo {};
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::NftInfo { token_id } => {
            let base_msg = Sg721QueryMsg::NftInfo { token_id };
            query_base::<Tile>(deps, env, base_msg)
        }
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => {
            let base_msg = Sg721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            };
            query_base::<Tile>(deps, env, base_msg)
        }
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => {
            let base_msg = Sg721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            };
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::AllTokens { start_after, limit } => {
            let base_msg = Sg721QueryMsg::AllTokens { start_after, limit };
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::Minter {} => {
            let base_msg = Sg721QueryMsg::Minter {};
            query_base::<Extension>(deps, env, base_msg)
        }
        QueryMsg::CollectionInfo {} => {
            let base_msg = Sg721QueryMsg::CollectionInfo {};
            query_base::<Extension>(deps, env, base_msg)
        }
    }
}

fn query_base<T>(deps: Deps, env: Env, msg: Sg721QueryMsg) -> StdResult<Binary>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    let base: Sg721Contract<T> = Sg721Contract::default();
    base.query(deps, env, msg)
}
