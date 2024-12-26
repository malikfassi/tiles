use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Empty};
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

// For incoming messages (from vending minter), use Extension (Option<Empty>)
pub type ExecuteMsg = sg721::ExecuteMsg<Extension, TileExecuteMsg>;

// For outgoing messages (to sg721), use Tile
pub type Sg721ExecuteMsg = sg721::ExecuteMsg<Tile, Empty>;

// Define our custom query messages
#[cw_serde]
pub enum QueryMsg {
    Config {},
    Base(sg721_base::msg::QueryMsg),
}
