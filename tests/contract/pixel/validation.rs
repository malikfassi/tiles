use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

#[test]
fn cannot_set_pixel_with_invalid_id() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 100, // Invalid pixel ID
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update]);
    test.assert_error_invalid_config(result, "Invalid pixel id: 100");

    Ok(())
}

#[test]
fn cannot_set_pixel_with_invalid_color() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "invalid".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update]);
    test.assert_error_invalid_config(result, "Invalid color format: invalid");

    Ok(())
}

#[test]
fn cannot_set_pixel_with_invalid_expiration() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3599, // Less than minimum
    };

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update]);
    test.assert_error_invalid_config(result, "Duration must be between 3600 and 86400 seconds");

    Ok(())
}

#[test]
fn batch_fails_with_one_invalid_id() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 100, // Invalid ID
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 50,
            color: "#0000FF".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    test.assert_error_invalid_config(result, "Invalid pixel id: 100");
    test.assert_token_hash(token_id, &initial_hash)?;

    Ok(())
}

#[test]
fn batch_fails_with_one_invalid_color() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 1,
            color: "invalid".to_string(), // Invalid color
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
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    test.assert_error_invalid_config(result, "Invalid color format: invalid");
    test.assert_token_hash(token_id, &initial_hash)?;

    Ok(())
}

#[test]
fn batch_fails_with_one_invalid_expiration_too_short() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration_duration: 3599, // Too short
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
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    test.assert_error_invalid_config(result, "Duration must be between 3600 and 86400 seconds");
    test.assert_token_hash(token_id, &initial_hash)?;

    Ok(())
}

#[test]
fn batch_fails_with_one_invalid_expiration_too_long() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration_duration: 86401, // Too long
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
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    test.assert_error_invalid_config(result, "Duration must be between 3600 and 86400 seconds");
    test.assert_token_hash(token_id, &initial_hash)?;

    Ok(())
}

#[test]
fn update_fails_with_duplicate_pixel_ids() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // Get initial balance
    let initial_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&owner, "ustars")?
        .amount
        .u128();

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

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    // Verify update failed
    test.assert_error_invalid_config(result, "Duplicate pixel id: 0");

    // Verify funds were refunded
    test.assert_funds_received(&owner, initial_balance, "ustars");

    Ok(())
} 