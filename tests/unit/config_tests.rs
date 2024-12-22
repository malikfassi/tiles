use cosmwasm_std::{Addr, Decimal, Uint128};
use tiles::state::PriceScaling;

use crate::common::fixtures::{setup_test, TestSetup};

#[test]
fn test_query_config() {
    let Ok(TestSetup {
        app,
        sender,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Query config
    let config = tiles.query_config(&app).unwrap();

    // Verify config values
    assert_eq!(config.admin, sender);
    assert_eq!(config.dev_address, sender);
    assert_eq!(config.dev_fee_percent, Decimal::percent(5));
    assert_eq!(config.base_price, Uint128::from(100_000u128));

    // Verify price scaling
    let price_scaling = config.price_scaling.unwrap();
    assert_eq!(price_scaling.hour_1_price, Uint128::from(100_000u128));
    assert_eq!(price_scaling.hour_12_price, Uint128::from(200_000u128));
    assert_eq!(price_scaling.hour_24_price, Uint128::from(300_000u128));
    assert_eq!(price_scaling.quadratic_base, Uint128::from(400_000u128));
}

#[test]
fn test_update_config() {
    let Ok(TestSetup {
        mut app,
        sender: _,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Query initial config
    let initial_config = tiles.query_config(&app).unwrap();

    // Update config
    let new_price_scaling = PriceScaling {
        hour_1_price: Uint128::from(500_000u128),
        hour_12_price: Uint128::from(600_000u128),
        hour_24_price: Uint128::from(700_000u128),
        quadratic_base: Uint128::from(800_000u128),
    };

    let res = tiles.update_config(
        &mut app,
        &initial_config.admin,
        None,
        None,
        None,
        Some(new_price_scaling.clone()),
    );
    assert!(res.is_ok());

    // Query updated config
    let updated_config = tiles.query_config(&app).unwrap();

    // Verify only price scaling was updated
    assert_eq!(updated_config.admin, initial_config.admin);
    assert_eq!(updated_config.dev_address, initial_config.dev_address);
    assert_eq!(updated_config.dev_fee_percent, initial_config.dev_fee_percent);
    assert_eq!(updated_config.base_price, initial_config.base_price);
    assert_eq!(updated_config.price_scaling.unwrap(), new_price_scaling);
}

#[test]
fn test_update_config_unauthorized() {
    let Ok(TestSetup {
        mut app,
        sender: _,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Create unauthorized address
    let unauthorized = Addr::unchecked("unauthorized");

    // Try to update config
    let res = tiles.update_config(
        &mut app,
        &unauthorized,
        None,
        Some(Decimal::percent(10)),
        Some(Uint128::from(200_000u128)),
        Some(PriceScaling {
            hour_1_price: Uint128::from(200_000u128),
            hour_12_price: Uint128::from(300_000u128),
            hour_24_price: Uint128::from(400_000u128),
            quadratic_base: Uint128::from(500_000u128),
        }),
    );
    assert!(res.is_err());
} 