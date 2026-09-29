//! Unit tests for the money rules: share configuration and payment split.

use cosmwasm_std::{Decimal, Uint128};
use tiles::contract::error::ContractError;
use tiles::core::validation::{split_payment, validate_shares, MAX_COLLECTION_SHARE_PERCENT};

#[test]
fn accepts_a_reasonable_split() {
    assert!(validate_shares(Decimal::percent(5), Decimal::percent(2)).is_ok());
    assert!(
        validate_shares(
            Decimal::percent(MAX_COLLECTION_SHARE_PERCENT),
            Decimal::zero()
        )
        .is_ok(),
        "the collection may take exactly its maximum"
    );
}

#[test]
fn rejects_a_collection_share_above_the_royalty_cap() {
    let result = validate_shares(
        Decimal::percent(MAX_COLLECTION_SHARE_PERCENT + 1),
        Decimal::zero(),
    );
    assert!(
        matches!(result, Err(ContractError::InvalidShareConfiguration { .. })),
        "expected InvalidShareConfiguration, got {:?}",
        result
    );
}

#[test]
fn rejects_a_split_over_one_hundred_percent() {
    let result = validate_shares(Decimal::percent(10), Decimal::percent(95));
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
    let (collection, platform, owner) = split_payment(total, Decimal::zero(), Decimal::zero());

    assert!(collection.is_zero());
    assert!(platform.is_zero());
    assert_eq!(owner, total);
}
