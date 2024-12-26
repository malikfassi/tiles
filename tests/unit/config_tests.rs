use cosmwasm_std::{Addr, Decimal};

use tiles::core::pricing::PriceScaling;

#[test]
fn test_update_config() {
    let mut setup = crate::common::helpers::setup::TestSetup::new();

    // Test valid config update
    setup
        .collection
        .update_config(
            &mut setup.app,
            &setup.admin,
            Some(setup.admin.to_string()),
            Some(Decimal::percent(5)),
            Some(PriceScaling {
                hour_1_price: 100_000_000u128.into(),
                hour_12_price: 200_000_000u128.into(),
                hour_24_price: 300_000_000u128.into(),
                quadratic_base: 400_000_000u128.into(),
            }),
        )
        .unwrap();
}

#[test]
fn test_update_config_unauthorized() {
    let mut setup = crate::common::helpers::setup::TestSetup::new();
    let unauthorized = Addr::unchecked("unauthorized");

    // Test unauthorized config update
    let err = setup
        .collection
        .update_config(
            &mut setup.app,
            &unauthorized,
            Some(unauthorized.to_string()),
            Some(Decimal::percent(5)),
            Some(PriceScaling {
                hour_1_price: 100_000_000u128.into(),
                hour_12_price: 200_000_000u128.into(),
                hour_24_price: 300_000_000u128.into(),
                quadratic_base: 400_000_000u128.into(),
            }),
        )
        .unwrap_err();

    assert_eq!(err.to_string(), "Unauthorized");
}
