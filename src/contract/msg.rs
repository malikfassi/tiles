use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Empty, Binary};
use sg721::{ExecuteMsg as Sg721ExecuteMsg, InstantiateMsg as Sg721InstantiateMsg};
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use crate::core::tile::Tile;

pub type InstantiateMsg = Sg721InstantiateMsg;

#[cw_serde]
pub enum ExecuteMsg {
    SetPixelColor {
        token_id: String,
        color: String,
        position: u32,
        expiration: u64,
    },
    UpdateConfig {
        dev_address: Option<String>,
        dev_fee_percent: Option<u64>,
        base_price: Option<u128>,
        price_scaling: Option<crate::contract::state::PriceScaling>,
    },
    Sg721(Sg721ExecuteMsg<Tile, Empty>),
}

#[cw_serde]
pub enum Extension {
    Config {},
    PixelState {
        token_id: String,
        position: u32,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::contract::state::Config)]
    Extension(Extension),
    #[returns(Binary)]
    Sg721(Sg721QueryMsg),
}

#[cw_serde]
pub struct PixelStateResponse {
    pub position: u32,
    pub color: String,
    pub expiration: u64,
    pub last_updated_by: String,
}
