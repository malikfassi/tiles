use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Empty};
use cw721_base::Extension;
use sg721::InstantiateMsg as Sg721InstantiateMsg;
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use sg721_base::msg::CollectionInfoResponse;

use crate::core::{
    pricing::PriceScaling,
    tile::{
        metadata::{PixelUpdate, TileMetadata},
        Tile,
    },
};

pub type InstantiateMsg = Sg721InstantiateMsg;

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
    #[returns(OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    #[returns(ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(OperatorsResponse)]
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(NumTokensResponse)]
    NumTokens {},
    #[returns(ContractInfoResponse)]
    ContractInfo {},
    #[returns(NftInfoResponse<Extension>)]
    NftInfo {
        token_id: String,
    },
    #[returns(AllNftInfoResponse<Extension>)]
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(cw721_base::MinterResponse)]
    Minter {},
    #[returns(CollectionInfoResponse)]
    CollectionInfo {},
    #[returns(PriceScaling)]
    PriceScaling {},
}
