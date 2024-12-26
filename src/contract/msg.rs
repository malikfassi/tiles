use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use sg721::ExecuteMsg as Sg721ExecuteMsg;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg721::InstantiateMsg as Sg721InstantiateMsg;

use crate::core::tile::metadata::{TileMetadata, PixelUpdate};
use crate::core::pricing::PriceScaling;
use crate::core::tile::Tile;

pub type InstantiateMsg = Sg721InstantiateMsg;

#[cw_serde]
pub enum ExecuteMsg {
    Base(Sg721ExecuteMsg<Tile, Empty>),
    Custom(CustomExecuteMsg),
}

#[cw_serde]
pub enum CustomExecuteMsg {
    SetPixelColor {
        token_id: String,
        current_metadata: TileMetadata,
        updates: Vec<PixelUpdate>,
    },
    UpdateConfig {
        tile_royalty_payment_address: Option<String>,
        tile_royalty_fee_percent: Option<cosmwasm_std::Decimal>,
        price_scaling: Option<PriceScaling>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Base(Sg721QueryMsg),
    Custom(CustomQueryMsg),
}

#[cw_serde]
pub enum CustomQueryMsg {
    Config {},
}
