use crate::core::pricing::PriceScaling;
use crate::core::tile::metadata::{PixelUpdate, TileMetadata};
use crate::core::tile::Tile;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};
use cw721::{DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg};

/// Collection extension message, used at instantiation and for `UpdateCollectionInfo`.
///
/// Canonical CW721 0.22 form: an optional collection extension holding description,
/// image, links and `royalty_info` (`payment_address` + `share`, capped at 0.10 by the
/// standard). This is the model Stargaze 2.0 documents on the Cosmos Hub.
pub type TilesCollectionExtensionMsg = DefaultOptionalCollectionExtensionMsg;

/// Collection extension read back through `GetCollectionInfo`.
pub type TilesCollectionExtensionRes = DefaultOptionalCollectionExtension;

#[cw_serde]
pub enum TileExecuteMsg {
    /// Pay to write the colour of one or more pixels on a tile.
    ///
    /// Anyone may call this: `sender` does not need to own the tile. A pixel with an
    /// active lease can only be written by the address that holds that lease (ADR 0004).
    SetPixelColor {
        token_id: String,
        /// The metadata the sender believes is current. Used as an optimistic lock.
        current_metadata: TileMetadata,
        updates: Vec<PixelUpdate>,
    },
    /// Update the duration-based price grid. Restricted to the collection payment address.
    UpdatePriceScaling(Box<PriceScaling>),
}

/// Execute message of the contract: the CW721 base messages plus our own extension messages.
pub type ExecuteMsg =
    cw721::msg::Cw721ExecuteMsg<Tile, TilesCollectionExtensionMsg, TileExecuteMsg>;

/// Query message of the contract: the CW721 base queries plus our own.
pub type QueryMsg = cw721::msg::Cw721QueryMsg<Tile, TilesCollectionExtensionRes, TileQueryMsg>;

/// Migrate message of the contract.
///
/// Empty on purpose: the storage layout is versioned with `cw2` and the contract reads
/// the installed version itself, so no migration input is needed from the caller. Anything
/// that did need input would have to be authorised, which this contract deliberately avoids.
#[cosmwasm_schema::cw_serde]
pub struct MigrateMsg {}

/// Custom queries added by the tiles contract.
#[cw_serde]
pub enum TileQueryMsg {
    /// Current duration-based price grid.
    PriceScaling {},
    /// Payment split and price floor configured at instantiation.
    Config {},
    /// The 100 pixels of a tile, without loading the whole token.
    TilePixels { token_id: String },
    /// What a set of pixel writes would cost on a given tile, before paying.
    QuotePixelUpdates {
        token_id: String,
        updates: Vec<PixelUpdate>,
    },
}

impl cw721::traits::Cw721CustomMsg for TileQueryMsg {}
impl cw721::traits::Cw721CustomMsg for TileExecuteMsg {}

/// Answer to `QuotePixelUpdates`: what the writes would cost right now.
#[cw_serde]
pub struct QuoteResponse {
    /// Total price to send with the transaction.
    pub total: Uint128,
    /// Amount that would go to the collection.
    pub collection_amount: Uint128,
    /// Amount that would go to the platform.
    pub platform_amount: Uint128,
    /// Amount that would go to the tile owner.
    pub owner_amount: Uint128,
    /// Expiration each pixel would receive, per update, in the order of the request.
    pub expirations: Vec<Timestamp>,
}
