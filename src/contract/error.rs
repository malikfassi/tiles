use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Base contract error: {0}")]
    Base(#[from] sg721_base::ContractError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidConfig: {0}")]
    InvalidConfig(String),

    #[error("HashMismatch")]
    HashMismatch {},

    #[error("InsufficientFunds")]
    InsufficientFunds {},

    #[error("InvalidFunds: sent amount is greater than required")]
    InvalidFunds {},
}
