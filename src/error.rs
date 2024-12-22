use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid price: {value}")]
    InvalidPrice { value: String },

    #[error("Invalid price scaling: {error}")]
    InvalidPriceScaling { error: String },

    #[error("Invalid color format: {color}")]
    InvalidColorFormat { color: String },

    #[error("Invalid pixel update - ID {id} exceeds max {max}")]
    InvalidPixelUpdate { id: u32, max: u32 },

    #[error("Invalid expiration - must be between {min} and {max}, got {value}")]
    InvalidExpiration { min: u64, max: u64, value: u64 },

    #[error("Batch size exceeded for {kind} - max {max}, got {got}")]
    BatchSizeExceeded { kind: String, max: u32, got: u32 },

    #[error("Hash mismatch - state has been modified")]
    HashMismatch {},

    #[error("Insufficient funds - required {required}, received {received}")]
    InsufficientFunds { required: Uint128, received: Uint128 },

    #[error("Pixel not expired - current time: {current}, expiration: {expiration}")]
    PixelNotExpired { current: u64, expiration: u64 },

    #[error("Invalid denomination - expected {expected}, received {received}")]
    InvalidDenom { expected: String, received: String },
}
