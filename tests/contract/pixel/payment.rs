use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;
use tiles::defaults::constants::DEFAULT_ROYALTY_SHARE;

#[test]
fn payment_is_split_correctly() -> Result<()> {
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

    let result = test.update_pixels(token_id, updates.clone(), &owner)?;

    // Assert all events
    for update in updates {
        test.assert_pixel_update_event(&result, &token_id.to_string(), &update, &owner);
    }
    test.assert_payment_distribution_event(
        &result,
        &token_id.to_string(),
        &owner,
        300000000 * DEFAULT_ROYALTY_SHARE as u128 / 100, // royalty amount
        300000000 * (100 - DEFAULT_ROYALTY_SHARE) as u128 / 100, // owner amount
    );

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

    let result = test.update_pixels(token_id, vec![update], &owner);
    test.assert_error_invalid_config(result, "Invalid pixel id: 100");
    test.assert_funds_received(&owner, initial_balance, "ustars");

    Ok(())
}


#[test]
fn duplicate_pixel_id_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 0, // Duplicate ID
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result = test.update_pixels(token_id, updates, &owner);
    test.assert_error_invalid_config(result, "Duplicate pixel id: 0");

    Ok(())
}
