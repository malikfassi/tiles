use crate::contract::error::ContractError;
use crate::contract::msg::QuoteResponse;
use crate::contract::state::Config;
use crate::core::pricing::PriceScaling;
use crate::core::tile::metadata::PixelUpdate;
use crate::core::validation::{split_payment, validate_updates};
use cosmwasm_std::Timestamp;

/// Computes what a set of pixel writes costs, and how it will be split.
///
/// This is the single source of truth for pricing: `SetPixelColor` and the
/// `QuotePixelUpdates` query both go through it, so a client that pays exactly what
/// it was quoted can never be rejected for a wrong amount.
pub fn quote(
    updates: &[PixelUpdate],
    price_scaling: &PriceScaling,
    config: &Config,
    now: Timestamp,
) -> Result<QuoteResponse, ContractError> {
    validate_updates(updates)?;

    let mut total = price_scaling
        .calculate_total_price(updates.iter().map(|update| &update.expiration_duration));
    if total < config.minimum_price {
        total = config.minimum_price;
    }

    let (collection_amount, platform_amount, owner_amount) = split_payment(
        total,
        config.collection_share_percent,
        config.platform_share_percent,
    );

    let expirations = updates
        .iter()
        .map(|update| update.expiration_timestamp(now))
        .collect();

    Ok(QuoteResponse {
        total,
        collection_amount,
        platform_amount,
        owner_amount,
        expirations,
    })
}
