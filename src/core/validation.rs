use crate::contract::error::ContractError;
use crate::core::tile::metadata::{PixelData, PixelUpdate, TileMetadata};
use crate::defaults::constants::{
    BPS_DENOMINATOR, PIXELS_PER_TILE, PIXEL_MAX_EXPIRATION, PIXEL_MIN_EXPIRATION,
};
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use std::collections::HashSet;

/// Maximum share (basis points) the collection may take from a pixel sale.
/// The CW721 collection extension caps royalties at 10 % (Stargaze 2.0 limit).
pub const MAX_COLLECTION_SHARE_BPS: u64 = 1_000;

/// Validates the shape of a single update: bounds, colour format, lease duration.
///
/// This is message validation. It never touches state, so it can be unit tested
/// without any chain.
pub fn validate_update(update: &PixelUpdate) -> Result<(), ContractError> {
    if (update.id as usize) >= PIXELS_PER_TILE {
        return Err(ContractError::InvalidPixelId { id: update.id });
    }

    let color = update.color.as_str();
    if !is_valid_hex_color(color) {
        return Err(ContractError::InvalidColorFormat {});
    }

    if update.expiration_duration < PIXEL_MIN_EXPIRATION {
        return Err(ContractError::InvalidLeaseTooShort {
            duration: update.expiration_duration,
            min: PIXEL_MIN_EXPIRATION,
        });
    }

    if update.expiration_duration > PIXEL_MAX_EXPIRATION {
        return Err(ContractError::InvalidLeaseTooLong {
            duration: update.expiration_duration,
            max: PIXEL_MAX_EXPIRATION,
        });
    }

    Ok(())
}

/// `#RRGGBB`, lowercase or uppercase, exactly seven characters.
pub fn is_valid_hex_color(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && color[1..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Validates that every update is well formed and that no pixel is targeted twice.
pub fn validate_updates(updates: &[PixelUpdate]) -> Result<(), ContractError> {
    if updates.is_empty() {
        return Err(ContractError::EmptyUpdates {});
    }

    let mut seen: HashSet<u8> = HashSet::with_capacity(updates.len());
    for update in updates {
        if !seen.insert(update.id) {
            return Err(ContractError::DuplicatePixelId { id: update.id });
        }
        validate_update(update)?;
    }

    Ok(())
}

/// Applies the lease rule (ADR 0004) to one update against the current pixel state.
///
/// - a pixel whose lease has expired, or that was never painted, may be written by anyone;
/// - a pixel under an active lease may only be written by the address holding that lease,
///   which covers the "extend my own colour" case as well as a fresh write by the same address.
pub fn validate_lease(
    token_id: &str,
    current: &PixelData,
    update: &PixelUpdate,
    sender: &Addr,
    now: Timestamp,
) -> Result<(), ContractError> {
    if !current.has_active_lease(now) {
        return Ok(());
    }

    if current.is_leased_by(sender) {
        return Ok(());
    }

    Err(ContractError::PixelLeaseActive {
        token_id: token_id.to_string(),
        pixel_id: update.id,
        expires_at: current.lease_expires_at.seconds(),
    })
}

/// Runs every rule that depends on the tile state, for the whole update list.
pub fn validate_updates_for_tile(
    token_id: &str,
    metadata: &TileMetadata,
    updates: &[PixelUpdate],
    sender: &Addr,
    now: Timestamp,
) -> Result<(), ContractError> {
    for update in updates {
        let current = &metadata.pixels[update.id as usize];
        validate_lease(token_id, current, update, sender, now)?;
    }
    Ok(())
}

/// Validates the payment split installed at instantiation.
pub fn validate_shares(
    collection_share_bps: u64,
    platform_share_bps: u64,
) -> Result<(), ContractError> {
    let total = collection_share_bps
        .checked_add(platform_share_bps)
        .ok_or_else(|| ContractError::Overflow("shares do not fit in u64".to_string()))?;

    if total > BPS_DENOMINATOR || collection_share_bps > MAX_COLLECTION_SHARE_BPS {
        return Err(ContractError::InvalidShareConfiguration {
            collection: format!("{} bp", collection_share_bps),
            platform: format!("{} bp", platform_share_bps),
        });
    }

    Ok(())
}

/// Splits `total` between collection, platform and tile owner.
///
/// The two shares are floored once each, and the owner receives the remainder, so the
/// three amounts always add up to `total` exactly whatever the rounding.
/// Basis points make the rounding explicit: on 12 345 uatom, 5 % is 617.25 → 617,
/// 2 % is 246.9 → 246, and the owner gets 12 345 - 617 - 246 = 11 482 instead of 11 481.9.
pub fn split_payment_bps(
    total: Uint128,
    collection_share_bps: u64,
    platform_share_bps: u64,
) -> (Uint128, Uint128, Uint128) {
    let denominator = Uint128::from(BPS_DENOMINATOR);
    let collection = total.multiply_ratio(collection_share_bps, denominator);
    let platform = total.multiply_ratio(platform_share_bps, denominator);
    (collection, platform, total - collection - platform)
}

/// Splits `total` between collection, platform and tile owner.
///
/// Thin wrapper on [`split_payment_bps`] for callers that hold shares as `Decimal`
/// fractions measured against the whole. Every share must stay within `[0, 1]`:
/// anything else would make the owner's remainder underflow.
pub fn split_payment(
    total: Uint128,
    collection_share_percent: Decimal,
    platform_share_percent: Decimal,
) -> (Uint128, Uint128, Uint128) {
    split_payment_bps(
        total,
        share_percent_to_bps(collection_share_percent),
        share_percent_to_bps(platform_share_percent),
    )
}

/// Converts a `Decimal` fraction of the whole into basis points, truncating.
///
/// A share above 100 %, or one that cannot be represented on the basis point scale,
/// is clamped to 10 000 bp: the owner then receives zero rather than the subtraction
/// underflowing. `validate_shares` rejects those values before any sale.
fn share_percent_to_bps(share_percent: Decimal) -> u64 {
    // A `Decimal` stores its value scaled by 10^18, so one whole unit is 10^18 atomics.
    const ONE: Uint128 = Uint128::new(1_000_000_000_000_000_000);
    let scaled = share_percent
        .atomics()
        .checked_mul(Uint128::from(BPS_DENOMINATOR))
        .map(|value| value / ONE)
        .unwrap_or(Uint128::from(BPS_DENOMINATOR));

    // Clamped to 10 000 before the cast, so the conversion is always lossless.
    scaled.min(Uint128::from(BPS_DENOMINATOR)).u128() as u64
}
