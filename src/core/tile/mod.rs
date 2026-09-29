use crate::core::tile::metadata::TileMetadata;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Env, MessageInfo};
use cw721::error::Cw721ContractError;
use cw721::traits::{Cw721CustomMsg, Cw721State, StateFactory};

pub mod metadata;

/// On-chain NFT extension. The pixels travel with the token, so the tile state
/// is owned by whoever owns the NFT.
#[cw_serde]
pub struct Tile {
    /// Canonical hash of `metadata`, used as an optimistic lock on writes.
    pub tile_hash: String,
    /// The 100 pixels of the tile.
    pub metadata: TileMetadata,
}

impl Cw721State for Tile {}
impl Cw721CustomMsg for Tile {}

/// Required by the CW721 query trait, used for extension-based filtering.
/// Tiles are not filtered by extension, so equality is the only meaningful case.
impl cw721::traits::Contains for Tile {
    fn contains(&self, other: &Self) -> bool {
        self == other
    }
}

/// `Tile` doubles as its own NFT extension message: the CW721 base asks the message
/// to produce the stored state. Writes go through `SetPixelColor`, so the base's
/// own state factory is a pass-through of the value it is given.
impl StateFactory<Tile> for Tile {
    fn create(
        &self,
        _deps: Deps,
        _env: &Env,
        _info: Option<&MessageInfo>,
        current: Option<&Tile>,
    ) -> Result<Tile, Cw721ContractError> {
        Ok(current.cloned().unwrap_or_else(|| self.clone()))
    }

    fn validate(
        &self,
        _deps: Deps,
        _env: &Env,
        _info: Option<&MessageInfo>,
        _current: Option<&Tile>,
    ) -> Result<(), Cw721ContractError> {
        Ok(())
    }
}
