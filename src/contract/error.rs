use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Base contract error: {0}")]
    Base(#[from] sg721_base::ContractError),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Fee error: {0}")]
    Fee(String),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Hash mismatch")]
    HashMismatch {},

    #[error("Invalid funds: {0}")]
    InvalidFunds(String),
}
