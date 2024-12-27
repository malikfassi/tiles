use crate::common::launchpad::Launchpad;
use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;
use tiles::defaults::constants::{DEFAULT_PRICE_1_HOUR, DEFAULT_PRICE_12_HOURS, DEFAULT_PRICE_24_HOURS};

#[test]
fn creator_can_update_price_scaling() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, PriceScaling::default());
    assert!(result.is_ok());
}

#[test]
fn buyer_cannot_update_price_scaling() {
    let mut ctx = Launchpad::new();
    let buyer = ctx.users.get_buyer();
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &buyer.address, PriceScaling::default());
    assert!(result.is_err());
}

#[test]
fn admin_cannot_update_price_scaling() {
    let mut ctx = Launchpad::new();
    let admin = ctx.users.admin();
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &admin, PriceScaling::default());
    assert!(result.is_err());
}

#[test]
fn cannot_set_hour_1_price_greater_than_hour_12_price() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();
    
    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
        hour_12_price: Uint128::from(DEFAULT_PRICE_1_HOUR),
        hour_24_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
        ..PriceScaling::default()
    };
    
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());
}

#[test]
fn cannot_set_hour_12_price_greater_than_hour_24_price() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();
    
    let invalid_scaling = PriceScaling {
        hour_12_price: Uint128::from(DEFAULT_PRICE_24_HOURS),
        hour_24_price: Uint128::from(DEFAULT_PRICE_12_HOURS),
        ..PriceScaling::default()
    };
    
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());
}

#[test]
fn cannot_set_zero_hour_1_price() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();
    
    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::zero(),
        ..PriceScaling::default()
    };
    
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());
}

#[test]
fn price_scaling_update_is_persisted() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();
    let new_scaling = PriceScaling::default();

    // Update price scaling
    ctx.tiles
        .update_price_scaling(&mut ctx.app, &creator, new_scaling.clone())
        .unwrap();

    // Query and verify
    let stored_scaling = ctx.tiles.query_price_scaling(&ctx.app).unwrap();
    assert_eq!(stored_scaling, new_scaling);
}

#[test]
fn price_scaling_update_emits_correct_event() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();

    let response = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, PriceScaling::default())
        .unwrap();

    // Verify wasm event exists with correct action
    let wasm_event = response.events.iter().find(|e| e.ty == "wasm").unwrap();
    assert!(wasm_event
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "update_price_scaling"));
}
