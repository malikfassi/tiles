use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;
use tiles::defaults::constants::DEFAULT_ROYALTY_SHARE;

#[test]
fn can_set_pixel_color() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test.update_pixels(token_id, vec![update.clone()], &owner)?;

    // Assert all events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &update, &owner);
    test.assert_payment_distribution_event(
        &result,
        &token_id.to_string(),
        &owner,
        100000000 * DEFAULT_ROYALTY_SHARE as u128 / 100, // royalty amount
        100000000 * (100 - DEFAULT_ROYALTY_SHARE) as u128 / 100, // owner amount
    );

    Ok(())
}

#[test]
fn all_valid_updates_succeed() -> Result<()> {
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

    // Assert all events for each update
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
fn can_update_multiple_pixels() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 49,
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 99,
            color: "#0000FF".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result = test.update_pixels(token_id, updates.clone(), &owner)?;

    // Assert all events for each update
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
