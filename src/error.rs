use cosmwasm_std::StdError;
use sg721_base::ContractError as Sg721ContractError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Sg721Error(#[from] Sg721ContractError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid color format: {color}")]
    InvalidColorFormat { color: String },

    #[error("Invalid expiration: min {min}, max {max}, got {value}")]
    InvalidExpiration { min: u64, max: u64, value: u64 },

    #[error("Invalid pixel update: id {id} exceeds max {max}")]
    InvalidPixelUpdate { id: u32, max: u32 },

    #[error("Invalid price: {value}")]
    InvalidPrice { value: String },

    #[error("Invalid price scaling: {error}")]
    InvalidPriceScaling { error: String },

    #[error("Hash mismatch")]
    HashMismatch {},

    #[error("Batch size exceeded: {kind} max {max}, got {got}")]
    BatchSizeExceeded { kind: String, max: u32, got: u32 },
}
