use anyhow::Result;
use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;
use tiles::defaults::constants::{
    DEFAULT_PRICE_12_HOURS, DEFAULT_PRICE_1_HOUR, DEFAULT_PRICE_24_HOURS,
};

use crate::common::TestOrchestrator;

#[test]
fn creator_can_update_price_scaling() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let creator = test.ctx.users.tile_contract_creator();

    let result =
        test.ctx
            .tiles
            .update_price_scaling(&mut test.ctx.app, &creator, PriceScaling::default());
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn buyer_cannot_update_price_scaling() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let buyer = test.ctx.users.get_buyer();

    let result = test.ctx.tiles.update_price_scaling(
        &mut test.ctx.app,
        &buyer.address,
        PriceScaling::default(),
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn admin_cannot_update_price_scaling() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let admin = test.ctx.users.admin();

    let result =
        test.ctx
            .tiles
            .update_price_scaling(&mut test.ctx.app, &admin, PriceScaling::default());
    assert!(result.is_err());

    Ok(())
}

#[test]
fn cannot_set_hour_1_price_greater_than_hour_12_price() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let creator = test.ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
        hour_12_price: Uint128::from(DEFAULT_PRICE_1_HOUR),
        hour_24_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
        ..PriceScaling::default()
    };

    let result = test
        .ctx
        .tiles
        .update_price_scaling(&mut test.ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn cannot_set_hour_12_price_greater_than_hour_24_price() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let creator = test.ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_12_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
        hour_24_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
        ..PriceScaling::default()
    };

    let result = test
        .ctx
        .tiles
        .update_price_scaling(&mut test.ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn cannot_set_zero_hour_1_price() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let creator = test.ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::zero(),
        ..PriceScaling::default()
    };

    let result = test
        .ctx
        .tiles
        .update_price_scaling(&mut test.ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn price_scaling_update_is_persisted() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let creator = test.ctx.users.tile_contract_creator();
    let new_scaling = PriceScaling::default();

    // Update price scaling
    test.ctx
        .tiles
        .update_price_scaling(&mut test.ctx.app, &creator, new_scaling.clone())?;

    // Query and verify
    let stored_scaling = test.ctx.tiles.query_price_scaling(&test.ctx.app)?;
    assert_eq!(stored_scaling, new_scaling);

    Ok(())
}

#[test]
fn price_scaling_update_emits_correct_event() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let creator = test.ctx.users.tile_contract_creator();

    let response = test.ctx.tiles.update_price_scaling(
        &mut test.ctx.app,
        &creator,
        PriceScaling::default(),
    )?;

    // Verify events
    let wasm = response
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .expect("No wasm event found");

    assert!(wasm
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "update_price_scaling"));

    Ok(())
}
