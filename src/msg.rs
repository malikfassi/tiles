use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Empty, QueryResponse, Uint128};
use sg721_base::msg::{CollectionInfoResponse, QueryMsg as Sg721QueryMsg};
use sg721::ExecuteMsg as Sg721ExecuteMsg;

use crate::types::{Extension, PriceScaling, TileMetadata};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub collection_info: CollectionInfoResponse,
    pub dev_address: String,
    pub dev_fee_percent: Decimal,
    pub base_price: Uint128,
    pub price_scaling: PriceScaling,
}

#[cw_serde]
pub enum ExecuteMsg {
    Sg721(Sg721ExecuteMsg<Extension, Empty>),
    SetPixelColor(SetPixelColorMsg),
    UpdateConfig(UpdateConfigMsg),
}

#[cw_serde]
pub struct SetPixelColorMsg {
    pub updates: Vec<TileUpdate>,
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
pub struct UpdateConfigMsg {
    pub dev_address: Option<String>,
    pub dev_fee_percent: Option<Decimal>,
    pub base_price: Option<Uint128>,
    pub price_scaling: Option<PriceScaling>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(TileStateResponse)]
    TileState { token_id: String },

    #[returns(QueryResponse)]
    Sg721(Sg721QueryMsg),
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub minter: String,
    pub dev_address: String,
    pub dev_fee_percent: Decimal,
    pub base_price: Uint128,
    pub price_scaling: PriceScaling,
}

#[cw_serde]
pub struct TileStateResponse {
    pub tile_hash: String,
    pub metadata: TileMetadata,
}
