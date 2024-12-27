use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Base contract error: {0}")]
    Base(#[from] sg721_base::ContractError),

    #[error("Unauthorized: sender '{sender}' is not allowed to perform this action")]
    Unauthorized { sender: String },

    #[error("Duplicate pixel ID: {id}")]
    DuplicatePixelId { id: u32 },

    #[error("Invalid pixel ID: {id} is out of bounds")]
    InvalidPixelId { id: u32 },

    #[error("Missing royalty info: contract was not initialized with royalty configuration")]
    MissingRoyaltyInfo {},

    #[error("Invalid pixel update: {reason}")]
    InvalidPixelUpdate { reason: String },

    #[error("Metadata hash mismatch: stored hash does not match provided metadata")]
    MetadataHashMismatch {},

    #[error("Insufficient funds: sent funds do not match required amount")]
    InsufficientFunds {},

    #[error("Overflow: {0}")]
    Overflow(String),
}

impl From<cw721_base::ContractError> for ContractError {
    fn from(err: cw721_base::ContractError) -> Self {
        ContractError::Base(sg721_base::ContractError::from(err))
    }
}

impl From<OverflowError> for ContractError {
    fn from(err: OverflowError) -> Self {
        ContractError::Overflow(err.to_string())
    }
}
