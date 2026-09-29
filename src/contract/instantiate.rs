use crate::{
    contract::{
        error::ContractError,
        state::{Config, CONFIG, PRICE_SCALING},
    },
    core::{pricing::PriceScaling, tile::Tile, validation::validate_shares},
    defaults::constants::{CONTRACT_NAME, CONTRACT_VERSION},
    events::{EventData, InstantiatePriceScalingEventData},
};
use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw721::traits::Cw721Execute;
use cw721::{
    extension::Cw721Extensions, DefaultOptionalCollectionExtension,
    DefaultOptionalCollectionExtensionMsg,
};
use serde_json;

/// NFT extension message: the tile itself is what gets written on mint and on
/// colour updates, so it doubles as its own message type.
pub type TileExtensionMsg = Tile;

/// Full contract type: on-chain NFT extension (`Tile`), collection extension
/// (description, image, royalties), our custom execute messages and queries.
pub type TilesContract<'a> = Cw721Extensions<
    'a,
    Tile,
    TileExtensionMsg,
    DefaultOptionalCollectionExtension,
    DefaultOptionalCollectionExtensionMsg,
    crate::contract::msg::TileExecuteMsg,
    crate::contract::msg::TileQueryMsg,
    Empty,
>;

/// Instantiate message of the contract.
pub type InstantiateMsg = cw721::msg::Cw721InstantiateMsg<DefaultOptionalCollectionExtensionMsg>;

pub fn instantiate_handler(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Install the CW721 base: collection info, royalties, minter, creator.
    let contract = TilesContract::default();
    contract.instantiate_with_version(
        deps.branch(),
        &env,
        &info,
        msg.clone(),
        CONTRACT_NAME,
        CONTRACT_VERSION,
    )?;

    let collection_share_percent = crate::defaults::constants::COLLECTION_SHARE_PERCENT;
    let platform_share_percent = crate::defaults::constants::PLATFORM_SHARE_PERCENT;
    validate_shares(collection_share_percent, platform_share_percent)?;

    let collection_payment_address = match msg.creator {
        Some(creator) => deps.api.addr_validate(&creator)?,
        None => info.sender.clone(),
    };

    let config = Config {
        collection_payment_address,
        collection_share_percent,
        platform_payment_address: info.sender.clone(),
        platform_share_percent,
        minimum_price: crate::defaults::constants::MIN_PIXEL_PRICE,
    };
    CONFIG.save(deps.storage, &config)?;

    let price_scaling = PriceScaling::default();
    PRICE_SCALING.save(deps.storage, &price_scaling)?;

    let config_event = InstantiatePriceScalingEventData {
        collection_info: serde_json::to_string(&msg.collection_info_extension).unwrap_or_default(),
        minter: msg.minter.unwrap_or_default(),
        price_scaling: serde_json::to_string(&price_scaling).unwrap_or_default(),
        time: env.block.time.to_string(),
    }
    .into_event();

    Ok(Response::new()
        .add_event(config_event)
        .add_attribute("method", "instantiate"))
}
