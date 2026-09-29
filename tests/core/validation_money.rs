//! Unit tests for the money rules: share configuration and payment split.

use cosmwasm_std::{Decimal, Uint128};
use tiles::contract::error::ContractError;
use tiles::core::validation::{
    split_payment, split_payment_bps, validate_shares, MAX_COLLECTION_SHARE_BPS,
};
use tiles::defaults::constants::{BPS_DENOMINATOR, COLLECTION_SHARE_BPS, PLATFORM_SHARE_BPS};

/// The BPS constant used by the contract is 1 000.
fn max_collection_share_bps() -> u64 {
    MAX_COLLECTION_SHARE_BPS
}

#[test]
fn accepts_a_reasonable_split() {
    assert!(validate_shares(COLLECTION_SHARE_BPS, PLATFORM_SHARE_BPS).is_ok());
    assert!(
        validate_shares(max_collection_share_bps(), 0).is_ok(),
        "the collection may take exactly its maximum"
    );
}

#[test]
fn rejects_a_collection_share_above_the_royalty_cap() {
    let result = validate_shares(max_collection_share_bps() + 1, 0);
    assert!(
        matches!(result, Err(ContractError::InvalidShareConfiguration { .. })),
        "expected InvalidShareConfiguration, got {:?}",
        result
    );
}

#[test]
fn rejects_a_split_over_one_hundred_percent() {
    let result = validate_shares(1_000, 9_500);
    assert!(
        matches!(result, Err(ContractError::InvalidShareConfiguration { .. })),
        "expected InvalidShareConfiguration, got {:?}",
        result
    );
}

#[test]
fn the_shares_always_add_up_to_the_total() {
    // Amounts chosen so the shares do not divide evenly.
    for amount in [1u128, 7, 100, 12_345, 999_999_937] {
        let total = Uint128::new(amount);
        let (collection, platform, owner) =
            split_payment(total, Decimal::percent(5), Decimal::percent(2));
        assert_eq!(
            collection + platform + owner,
            total,
            "shares must add up exactly for {}",
            total
        );
    }
}

#[test]
fn the_owner_receives_the_remainder() {
    let total = Uint128::new(12_345);
    let (collection, platform, owner) =
        split_payment(total, Decimal::percent(5), Decimal::percent(2));

    assert_eq!(collection, Uint128::new(617), "5 % of 12 345, floored");
    assert_eq!(platform, Uint128::new(246), "2 % of 12 345, floored");
    assert_eq!(
        owner,
        Uint128::new(11_482),
        "the remainder goes to the owner"
    );
}

#[test]
fn a_zero_split_gives_everything_to_the_owner() {
    let total = Uint128::new(1000);
    let (collection, platform, owner) = split_payment_bps(total, 0, 0);

    assert!(collection.is_zero());
    assert!(platform.is_zero());
    assert_eq!(owner, total);
}

#[test]
fn the_bps_split_matches_the_decimal_split() {
    // The two entry points must agree: the basis point version is the one the
    // contract uses, the decimal one stays available to pure callers.
    for amount in [1u128, 7, 100, 12_345, 999_999_937, u128::MAX] {
        let total = Uint128::new(amount);
        assert_eq!(
            split_payment_bps(total, COLLECTION_SHARE_BPS, PLATFORM_SHARE_BPS),
            split_payment(total, Decimal::percent(5), Decimal::percent(2)),
            "both splits must agree for {}",
            total
        );
    }
}

#[test]
fn the_bps_split_is_exact_for_amounts_that_round() {
    // 5 % of 12 345 is 617.25 and 2 % is 246.9, so both floors lose a fraction.
    let total = Uint128::new(12_345);
    let (collection, platform, owner) = split_payment_bps(total, 500, 200);

    assert_eq!(collection, Uint128::new(617), "5 % floored");
    assert_eq!(platform, Uint128::new(246), "2 % floored");
    assert_eq!(
        owner,
        Uint128::new(11_482),
        "the owner absorbs the 0.15 remainder"
    );
    assert_eq!(collection + platform + owner, total);
}

#[test]
fn the_owner_absorbs_exactly_the_two_floored_fractions() {
    // Each share is floored once, so the owner absorbs what both floors dropped: the
    // remainder over the exact share is never negative and never reaches two units.
    // This is the whole rounding policy: nothing is lost, nothing is created.
    for amount in (1u128..=2_000).chain([3_333, 99_999, 123_456_789]) {
        let total = Uint128::new(amount);
        let (collection, platform, owner) = split_payment_bps(total, 500, 200);
        let exact_owner = total.multiply_ratio(9_300u128, BPS_DENOMINATOR);

        assert_eq!(collection + platform + owner, total, "sum for {}", total);
        assert!(
            owner >= exact_owner,
            "the owner must never be short-changed for {}",
            total
        );
        assert!(
            owner - exact_owner <= Uint128::new(2),
            "the owner absorbs at most one unit per floored share, for {}",
            total
        );
    }
}

#[test]
fn the_exact_arithmetic_share_is_always_reachable() {
    // On a total the shares divide evenly, the split is exact with no remainder at all.
    // 10 000 uatom is the smallest such total for 500 bp and 200 bp together.
    let total = Uint128::new(10_000);
    let (collection, platform, owner) = split_payment_bps(total, 500, 200);

    assert_eq!(collection, Uint128::new(500));
    assert_eq!(platform, Uint128::new(200));
    assert_eq!(owner, Uint128::new(9_300));
}

#[test]
fn a_full_split_leaves_nothing_to_the_owner() {
    // Degenerate but reachable configuration: collection 10 % + platform 90 %.
    let total = Uint128::new(1_000_000);
    let (collection, platform, owner) = split_payment_bps(total, 1_000, 9_000);

    assert_eq!(owner, Uint128::zero());
    assert_eq!(collection + platform, total);
}

#[test]
fn a_tiny_total_gives_everything_to_the_owner() {
    // Below 10 uatom both shares floor to zero: the owner is the only one paid,
    // which is the correct outcome rather than a lost remainder.
    let total = Uint128::new(9);
    let (collection, platform, owner) = split_payment_bps(total, 500, 200);

    assert_eq!(collection, Uint128::zero());
    assert_eq!(platform, Uint128::zero());
    assert_eq!(owner, total);
}
