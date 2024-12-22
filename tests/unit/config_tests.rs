use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_multi_test::Executor;

use tiles::defaults::config::{
    DEFAULT_BASE_PRICE, DEFAULT_DEV_FEE_PERCENT, DEFAULT_PRICE_SCALING,
};
use tiles::types::PriceScaling;

use crate::common::setup::setup_test;

#[test]
fn test_query_config() {
    let (mut app, tiles) = setup_test();

    // Query config
    let config = tiles.query_config(&app).unwrap();

    // Verify config values match defaults
    assert_eq!(config.admin, Addr::unchecked("contract0"));
    assert_eq!(config.minter, Addr::unchecked("contract1"));
    assert_eq!(config.tiles_royalty_payment_address, Addr::unchecked("contract1"));
    assert_eq!(config.tiles_royalties, DEFAULT_TILES_ROYALTIES);

    // Verify price scaling
    let price_scaling = config.price_scaling;
    assert_eq!(price_scaling.hour_1_price, DEFAULT_PRICE_SCALING.hour_1_price);
    assert_eq!(price_scaling.hour_12_price, DEFAULT_PRICE_SCALING.hour_12_price);
    assert_eq!(price_scaling.hour_24_price, DEFAULT_PRICE_SCALING.hour_24_price);
    assert_eq!(
        price_scaling.quadratic_base,
        DEFAULT_PRICE_SCALING.quadratic_base
    );
}

#[test]
fn test_update_config() {
    let (mut app, tiles) = setup_test();

    // Query initial config
    let initial_config = tiles.query_config(&app).unwrap();

    // Update config with doubled prices
    let new_price_scaling = PriceScaling {
        hour_1_price: initial_config.price_scaling.hour_1_price * Uint128::from(2u128),
        hour_12_price: initial_config.price_scaling.hour_12_price * Uint128::from(2u128),
        hour_24_price: initial_config.price_scaling.hour_24_price * Uint128::from(2u128),
        quadratic_base: initial_config.price_scaling.quadratic_base * Uint128::from(2u128),
    };

    let res = tiles.update_config(
        &mut app,
        &Addr::unchecked("contract0"), // Admin address
        None,
        None,
        Some(new_price_scaling.clone()),
    );
    assert!(res.is_ok());

    // Query updated config
    let updated_config = tiles.query_config(&app).unwrap();

    // Verify only price scaling was updated
    assert_eq!(updated_config.admin, initial_config.admin);
    assert_eq!(updated_config.minter, initial_config.minter);
    assert_eq!(updated_config.tiles_royalty_payment_address, initial_config.tiles_royalty_payment_address);
    assert_eq!(
        updated_config.tiles_royalties,
        initial_config.tiles_royalties
    );
    assert_eq!(updated_config.price_scaling, new_price_scaling);
}

#[test]
fn test_update_config_unauthorized() {
    let (mut app, tiles) = setup_test();

    // Try to update config with non-admin address
    let res = tiles.update_config(
        &mut app,
        &Addr::unchecked("unauthorized"),
        None,
        None,
        Some(DEFAULT_PRICE_SCALING),
    );
    assert!(res.is_err());
}
