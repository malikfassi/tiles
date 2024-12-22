use cosmwasm_std::Decimal;

use crate::error::ContractError;
use crate::msg::UpdateConfigMsg;

/// Validates config update parameters
pub fn validate_config_update(msg: &UpdateConfigMsg) -> Result<(), ContractError> {
    if let Some(dev_fee_percent) = msg.dev_fee_percent {
        // Ensure fee percent is between 0 and 100
        if dev_fee_percent > Decimal::percent(100) || dev_fee_percent < Decimal::zero() {
            return Err(ContractError::InvalidFeePercent {});
        }
    }
    Ok(())
} 