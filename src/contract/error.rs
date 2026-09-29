use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Cw721(#[from] cw721::error::Cw721ContractError),

    #[error("Unauthorized: sender is not allowed to perform this action")]
    Unauthorized {},

    #[error("Overflow: {0}")]
    Overflow(String),

    // ---- Message validation ----
    #[error("Pixel id {id} is out of bounds (0..{max})", max = crate::defaults::constants::PIXELS_PER_TILE)]
    InvalidPixelId { id: u8 },

    #[error("Invalid colour format: expected #RRGGBB")]
    InvalidColorFormat {},

    #[error("Duplicate pixel id {id} in the same message")]
    DuplicatePixelId { id: u8 },

    #[error("Lease duration {duration} is below the minimum of {min} seconds")]
    InvalidLeaseTooShort { duration: u64, min: u64 },

    #[error("Lease duration {duration} is above the maximum of {max} seconds")]
    InvalidLeaseTooLong { duration: u64, max: u64 },

    #[error("Empty update list: nothing to colour")]
    EmptyUpdates {},

    // ---- State and business rules ----
    #[error("Tile {token_id} not found")]
    TileNotFound { token_id: String },

    #[error("Pixel {pixel_id} of tile {token_id} is leased until {expires_at} by another address")]
    PixelLeaseActive {
        token_id: String,
        pixel_id: u8,
        expires_at: u64,
    },

    #[error("Tile metadata changed since it was read: the provided hash is stale")]
    MetadataHashMismatch {},

    #[error("Collection royalties are not configured")]
    MissingRoyaltyInfo {},

    // ---- Payments ----
    #[error("Payment must be a single coin matching the required amount of {expected}")]
    InvalidPayment { expected: String },

    #[error("Invalid price scaling: prices must be positive and increasing")]
    InvalidPriceScaling {},

    #[error("Share configuration is invalid: collection {collection}% + platform {platform}% must not exceed 100%")]
    InvalidShareConfiguration {
        collection: String,
        platform: String,
    },

    // ---- Migration ----
    #[error("Migration from contract version {from} is not supported")]
    UnsupportedMigration { from: String },
}

impl From<OverflowError> for ContractError {
    fn from(err: OverflowError) -> Self {
        ContractError::Overflow(err.to_string())
    }
}
