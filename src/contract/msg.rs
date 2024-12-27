use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Empty};
use cw721_base::Extension;

use crate::core::{
    pricing::PriceScaling,
    tile::{
        metadata::{PixelUpdate, TileMetadata},
        Tile,
    },
};

// Use sg721's InstantiateMsg directly
pub type InstantiateMsg = sg721::InstantiateMsg;

#[cw_serde]
pub enum TileExecuteMsg {
    SetPixelColor {
        token_id: String,
        current_metadata: TileMetadata,
        updates: Vec<PixelUpdate>,
    },
    UpdatePriceScaling(PriceScaling),
}

// For incoming messages (from vending minter), use Extension (Option<Empty>)
pub type ExecuteMsg = sg721::ExecuteMsg<Extension, TileExecuteMsg>;

// For outgoing messages (to sg721), use Tile
pub type Sg721ExecuteMsg = sg721::ExecuteMsg<Tile, Empty>;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceScaling)]
    PriceScaling {},
    #[returns(Binary)]
    Base(sg721_base::msg::QueryMsg),
}
