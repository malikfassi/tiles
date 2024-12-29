use anyhow::Result;
use cosmwasm_std::Uint128;
use tiles::{
    core::pricing::PriceScaling,
    defaults::constants::{
        DEFAULT_PRICE_1_HOUR, DEFAULT_PRICE_12_HOURS, DEFAULT_PRICE_24_HOURS,
    },
};

use crate::common::{EventAssertions, TestContext};

#[test]
fn creator_can_update_price_scaling() -> Result<()> {
    let mut ctx = TestContext::new();
    let creator = ctx.users.tile_contract_creator();

    let result = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &creator.address,
        PriceScaling::default(),
    );
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn pixel_operator_cannot_update_price_scaling() -> Result<()> {
    let mut ctx = TestContext::new();
    let operator = ctx.users.pixel_operator();

    let result = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &operator.address,
        PriceScaling::default(),
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn buyer_cannot_update_price_scaling() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer();

    let result = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &buyer.address,
        PriceScaling::default(),
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn cannot_set_hour_1_price_greater_than_hour_12_price() -> Result<()> {
    let mut ctx = TestContext::new();
    let creator = ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
        hour_12_price: Uint128::from(DEFAULT_PRICE_1_HOUR),
        hour_24_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
        quadratic_base: Uint128::from(1u128),
    };

    let result = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &creator.address,
        invalid_scaling,
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn cannot_set_hour_12_price_greater_than_hour_24_price() -> Result<()> {
    let mut ctx = TestContext::new();
    let creator = ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_12_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
        hour_24_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
        quadratic_base: Uint128::from(1u128),
        ..PriceScaling::default()
    };

    let result = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &creator.address,
        invalid_scaling,
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn cannot_set_zero_hour_1_price() -> Result<()> {
    let mut ctx = TestContext::new();
    let creator = ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::zero(),
        quadratic_base: Uint128::from(1u128),
        ..PriceScaling::default()
    };

    let result = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &creator.address,
        invalid_scaling,
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn price_scaling_update_is_persisted() -> Result<()> {
    let mut ctx = TestContext::new();
    let creator = ctx.users.tile_contract_creator();
    let new_scaling = PriceScaling::default();

    // Update price scaling
    ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &creator.address,
        new_scaling.clone(),
    )?;

    // Query and verify
    let stored_scaling = ctx.tiles.query_price_scaling(&ctx.app)?;
    assert_eq!(stored_scaling, new_scaling);

    Ok(())
}

#[test]
fn price_scaling_update_emits_correct_event() -> Result<()> {
    let mut ctx = TestContext::new();
    let creator = ctx.users.tile_contract_creator();

    let new_scaling = PriceScaling::default();
    let response = ctx.tiles.update_price_scaling(
        &mut ctx.app,
        &creator.address,
        new_scaling.clone(),
    )?;

    // Assert the event
    EventAssertions::assert_price_scaling_update(&response, &new_scaling);

    Ok(())
}
