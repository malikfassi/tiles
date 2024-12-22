use cosmwasm_std::{BankMsg, Coin, MessageInfo, Response, Uint128};
use sg_std::StargazeMsgWrapper;

use crate::error::ContractError;
use crate::types::Config;
use crate::utils::events::payment_attributes;

/// Core payment information
#[derive(Debug)]
pub struct PaymentInfo {
    pub total_amount: Uint128,
    pub royalty_fee: Uint128,
    pub royalty_address: String,
}

/// Process and validate payment
pub fn process_payment(
    info: &MessageInfo,
    required_amount: Uint128,
    config: &Config,
) -> Result<PaymentInfo, ContractError> {
    // Get payment amount from funds
    let payment_amount = info
        .funds
        .iter()
        .find(|c| c.denom == "ustars")
        .map(|c| c.amount)
        .unwrap_or_default();

    // Validate payment amount
    if payment_amount < required_amount {
        return Err(ContractError::InsufficientFunds {
            required: required_amount,
            received: payment_amount,
        });
    }

    // Calculate royalty fee
    let royalty_fee = required_amount * config.tiles_royalties;

    Ok(PaymentInfo {
        total_amount: payment_amount,
        royalty_fee,
        royalty_address: config.tiles_royalty_payment_address.to_string(),
    })
}

/// Create payment response with messages and events
pub fn create_payment_response(
    payment_info: &PaymentInfo,
) -> Response<StargazeMsgWrapper> {
    let mut response = Response::new();

    // Add royalty payment if needed
    if !payment_info.royalty_fee.is_zero() {
        response = response.add_message(BankMsg::Send {
            to_address: payment_info.royalty_address.clone(),
            amount: vec![Coin::new(payment_info.royalty_fee.u128(), "ustars")],
        });
    }

    // Add payment event attributes
    response.add_attributes(payment_attributes(
        payment_info.total_amount.u128(),
        "ustars",
        Some(payment_info.royalty_fee.u128()),
    ))
}

/// Validate coin denomination
pub fn validate_denom(coin: &Coin) -> Result<(), ContractError> {
    if coin.denom != "ustars" {
        return Err(ContractError::InvalidDenom {
            expected: "ustars".to_string(),
            received: coin.denom.clone(),
        });
    }
    Ok(())
} 