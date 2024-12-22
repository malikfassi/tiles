use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Decimal, Empty, Uint128};
use sg721::{CollectionInfo, ExecuteMsg as Sg721ExecuteMsg, InstantiateMsg as Sg721InstantiateMsg, RoyaltyInfoResponse};
use sg721_base::msg::QueryMsg as Sg721QueryMsg;

use crate::state::{PriceScaling, TileState};

#[cw_serde]
#[derive(Default)]
pub struct Extension {
    pub tile_hash: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub collection_info: CollectionInfo<RoyaltyInfoResponse>,
    pub dev_address: String,
    pub dev_fee_percent: Decimal,
    pub base_price: Uint128,
    pub price_scaling: PriceScaling,
}

impl From<InstantiateMsg> for Sg721InstantiateMsg {
    fn from(msg: InstantiateMsg) -> Self {
        Sg721InstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
            collection_info: msg.collection_info,
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPixelColor(SetPixelColorMsg),
    UpdateConfig {
        dev_address: Option<String>,
        dev_fee_percent: Option<Decimal>,
        base_price: Option<Uint128>,
        price_scaling: Option<PriceScaling>,
    },
    Base(Sg721ExecuteMsg<Extension, Empty>),
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
    pub max_message_size: u32,
    pub updates: Vec<TileUpdate>,
}

#[cw_serde]
pub struct TileUpdate {
    pub tile_id: String,
    pub current_metadata: TileMetadata,
    pub updates: TileUpdates,
}

#[cw_serde]
pub struct TileMetadata {
    pub tile_id: String,
    pub pixels: Vec<PixelData>,
}

#[cw_serde]
pub struct TileUpdates {
    pub pixels: Vec<PixelUpdate>,
}

#[cw_serde]
pub struct PixelData {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
    pub last_updated_by: String,
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
    #[returns(crate::state::Config)]
    Config {},
    #[returns(TileState)]
    TileState { token_id: String },
    #[returns(Binary)]
    Base(Sg721QueryMsg),
} 