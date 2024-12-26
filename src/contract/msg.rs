use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;
use sg721_base::msg::ExecuteMsg as Sg721BaseExecuteMsg;

use crate::core::{
    pricing::PriceScaling,
    tile::{metadata::{PixelUpdate, TileMetadata}, Tile},
};

// Use sg721's InstantiateMsg directly
pub type InstantiateMsg = sg721::InstantiateMsg;

// Define our custom extension messages
#[cw_serde]
pub enum TileExecuteMsg {
    SetPixelColor {
        token_id: String,
        current_metadata: TileMetadata,
        updates: Vec<PixelUpdate>,
    },
    UpdateConfig {
        tile_royalty_payment_address: Option<String>,
        tile_royalty_fee_percent: Option<Decimal>,
        price_scaling: Option<PriceScaling>,
    },
}

// Use sg721_base's ExecuteMsg with our extension
pub type ExecuteMsg = Sg721BaseExecuteMsg<Tile, TileExecuteMsg>;

// Define our custom query messages
#[cw_serde]
pub enum QueryMsg {
    Config {},
    Base(sg721_base::msg::QueryMsg),
}
