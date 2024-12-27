use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

#[test]
fn payment_is_distributed_correctly_for_single_update() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update])?;

    println!("Single update test response: {:?}", result);
    test.assert_pixel_update_payment(&result, &token_id.to_string(), 100000000);

    Ok(())
}

#[test]
fn payment_is_distributed_correctly_for_multiple_updates() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 2,
            color: "#0000FF".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates)?;

    println!("Multiple updates test response: {:?}", result);
    test.assert_pixel_update_payment(&result, &token_id.to_string(), 300000000);

    Ok(())
}

#[test]
fn update_fails_with_insufficient_funds() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let poor_user = test.ctx.users.poor_user();
    let poor_user_addr = &poor_user.address;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel_with_funds(&mut test.ctx.app, poor_user_addr, token_id, vec![update], 50000000);

    test.assert_error_insufficient_funds(result);

    Ok(())
}

#[test]
fn update_fails_with_excess_funds() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel_with_funds(&mut test.ctx.app, &owner, token_id, vec![update], 200000000);

    test.assert_error_invalid_funds(result);

    Ok(())
}

#[test]
fn payment_is_refunded_when_update_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&owner, "ustars")?
        .amount
        .u128();

    let update = PixelUpdate {
        id: 100, // Invalid ID
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update]);

    test.assert_error_invalid_config(result, "Invalid pixel id: 100");
    test.assert_funds_received(&owner, initial_balance, "ustars");

    Ok(())
} 