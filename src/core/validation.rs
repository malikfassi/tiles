use crate::contract::error::ContractError;
use crate::core::tile::metadata::{PixelData, PixelUpdate, TileMetadata};
use crate::defaults::constants::{PIXELS_PER_TILE, PIXEL_MAX_EXPIRATION, PIXEL_MIN_EXPIRATION};
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use std::collections::HashSet;

/// Maximum share (percent) the collection may take from a pixel sale.
/// The CW721 collection extension caps royalties at 10 % (Stargaze 2.0 limit).
pub const MAX_COLLECTION_SHARE_PERCENT: u64 = 10;

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
    collection_share_percent: Decimal,
    platform_share_percent: Decimal,
) -> Result<(), ContractError> {
    let total = collection_share_percent
        .checked_add(platform_share_percent)
        .map_err(|e| ContractError::Overflow(e.to_string()))?;

    if total > Decimal::one()
        || collection_share_percent > Decimal::percent(MAX_COLLECTION_SHARE_PERCENT)
    {
        return Err(ContractError::InvalidShareConfiguration {
            collection: collection_share_percent.to_string(),
            platform: platform_share_percent.to_string(),
        });
    }

    Ok(())
}

/// Splits `total` between collection, platform and tile owner.
///
/// The owner receives the remainder, so the three amounts always add up to `total`
/// exactly, whatever the rounding of the two shares (ADR 0004, TODO.md §9).
pub fn split_payment(
    total: Uint128,
    collection_share_percent: Decimal,
    platform_share_percent: Decimal,
) -> (Uint128, Uint128, Uint128) {
    let collection = total.mul_floor(collection_share_percent);
    let platform = total.mul_floor(platform_share_percent);
    let owner = total - collection - platform;
    (collection, platform, owner)
}
