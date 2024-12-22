use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Base(String),

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

    #[error("Unauthorized")]
    Unauthorized {},
}

impl From<sg721_base::ContractError> for ContractError {
    fn from(err: sg721_base::ContractError) -> Self {
        ContractError::Base(err.to_string())
    }
}
  