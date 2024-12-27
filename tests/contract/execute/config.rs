use crate::common::launchpad::Launchpad;
use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;

#[test]
fn test_update_price_scaling() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();

    let new_scaling = PriceScaling {
        hour_1_price: Uint128::from(100u128),
        hour_12_price: Uint128::from(200u128),
        hour_24_price: Uint128::from(300u128),
        quadratic_base: Uint128::from(400u128),
    };

    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, new_scaling);
    assert!(result.is_ok());
}

#[test]
fn test_update_price_scaling_unauthorized() {
    let mut ctx = Launchpad::new();
    let unauthorized = ctx.users.get_buyer();

    let new_scaling = PriceScaling {
        hour_1_price: Uint128::from(100u128),
        hour_12_price: Uint128::from(200u128),
        hour_24_price: Uint128::from(300u128),
        quadratic_base: Uint128::from(400u128),
    };

    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &unauthorized.address, new_scaling);
    assert!(result.is_err());
}

#[test]
fn test_update_price_scaling_invalid() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();

    let invalid_scaling = PriceScaling {
        hour_1_price: Uint128::from(400u128), // Invalid: hour_1_price > hour_12_price
        hour_12_price: Uint128::from(200u128),
        hour_24_price: Uint128::from(300u128),
        quadratic_base: Uint128::from(400u128),
    };

    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, invalid_scaling);
    assert!(result.is_err());
}

#[test]
fn test_update_price_scaling_as_creator() {
    let mut ctx = Launchpad::new();
    let creator = ctx.users.tile_contract_creator();

    let new_price_scaling = PriceScaling {
        hour_1_price: Uint128::new(100),
        hour_12_price: Uint128::new(200),
        hour_24_price: Uint128::new(300),
        quadratic_base: Uint128::new(400),
    };

    // Update price scaling as creator (who is also the payment address)
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &creator, new_price_scaling.clone());
    assert!(result.is_ok());

    // Verify the update
    let updated_price_scaling = ctx.tiles.query_price_scaling(&ctx.app).unwrap();
    assert_eq!(updated_price_scaling, new_price_scaling);
}

#[test]
fn test_update_price_scaling_as_unauthorized() {
    let mut ctx = Launchpad::new();
    let unauthorized = ctx.users.user1();

    let new_price_scaling = PriceScaling {
        hour_1_price: Uint128::new(100),
        hour_12_price: Uint128::new(200),
        hour_24_price: Uint128::new(300),
        quadratic_base: Uint128::new(400),
    };

    // Try to update price scaling as unauthorized user
    let result = ctx
        .tiles
        .update_price_scaling(&mut ctx.app, &unauthorized, new_price_scaling);
    assert!(result.is_err());
}
