use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Decimal, Empty, Uint128};
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sg721_base::msg::QueryMsg as Sg721QueryMsg;

use crate::state::{PriceScaling, Extension, TileMetadata};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub collection_info: CollectionInfo<RoyaltyInfoResponse>,
}

#[cw_serde]
pub enum ExecuteMsg {
    #[serde(rename = "mint")]
    Mint {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Option<Extension>,
    },
    SetPixelColor(SetPixelColorMsg),
    UpdateConfig(UpdateConfigMsg),
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub dev_address: Option<String>,
    pub dev_fee_percent: Option<Decimal>,
    pub base_price: Option<Uint128>,
    pub price_scaling: Option<PriceScaling>,
}

#[cw_serde]
pub struct SetPixelColorMsg {
    pub updates: Vec<TileUpdate>,
    pub max_message_size: u32,
}

#[cw_serde]
pub struct TileUpdate {
    pub tile_id: String,
    pub current_metadata: TileMetadata,
    pub updates: TileUpdates,
}

#[cw_serde]
pub struct TileUpdates {
    pub pixels: Vec<PixelUpdate>,
}

#[cw_serde]
pub struct PixelUpdate {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Sg721Base(Sg721QueryMsg),
    #[returns(crate::state::Config)]
    Config {},
} 