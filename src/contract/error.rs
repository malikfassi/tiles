use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid color format")]
    InvalidColorFormat {},

    #[error("Invalid expiration")]
    InvalidExpiration {},

    #[error("Invalid pixel position")]
    InvalidPixelPosition {},

    #[error("Invalid dev fee percent")]
    InvalidDevFeePercent {},

    #[error("Invalid price scaling")]
    InvalidPriceScaling {},

    #[error("Hash mismatch - state has been modified")]
    HashMismatch {},

    #[error("Invalid pixel update")]
    InvalidPixelUpdate {},

    #[error("Message too large")]
    MessageTooLarge {},
}
