use cosmwasm_std::StdError;
use sg721_base::ContractError as Sg721ContractError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Hash mismatch - state has been modified")]
    HashMismatch {},

    #[error("Invalid pixel update")]
    InvalidPixelUpdate {},

    #[error("Invalid color format")]
    InvalidColorFormat {},

    #[error("Invalid expiration")]
    InvalidExpiration {},

    #[error("Message too large")]
    MessageTooLarge {},

    #[error("Sg721 error: {0}")]
    Sg721Error(#[from] Sg721ContractError),
}
  