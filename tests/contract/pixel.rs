use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};

#[test]
fn can_set_pixel_color() -> Result<()> {
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

    // Verify events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    Ok(())
}

#[test]
fn all_valid_updates_succeed() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let updates = vec![
        // Valid update with minimum expiration
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        // Valid update with maximum expiration
        PixelUpdate {
            id: 99,
            color: "#00FF00".to_string(),
            expiration_duration: 86400,
        },
        // Valid update with mid-range expiration
        PixelUpdate {
            id: 50,
            color: "#0000FF".to_string(),
            expiration_duration: 43200,
        },
        // Valid update with different color format
        PixelUpdate {
            id: 25,
            color: "#123456".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result =
        test.ctx
            .tiles
            .update_pixel(&mut test.ctx.app, &owner, token_id, updates.clone())?;

    // Verify events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    // Calculate expected hash
    let mut expected_metadata = TileMetadata::default();
    expected_metadata.apply_updates(
        updates,
        &owner,
        test.ctx.app.inner().block_info().time.seconds(),
    );
    test.assert_token_hash(token_id, &expected_metadata.hash())?;

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
            id: 99,
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result =
        test.ctx
            .tiles
            .update_pixel(&mut test.ctx.app, &owner, token_id, updates.clone())?;

    // Verify events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    // Calculate expected hash
    let mut expected_metadata = TileMetadata::default();
    expected_metadata.apply_updates(
        updates,
        &owner,
        test.ctx.app.inner().block_info().time.seconds(),
    );
    test.assert_token_hash(token_id, &expected_metadata.hash())?;

    Ok(())
}

#[test]
fn hash_remains_unchanged_when_update_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // Get initial hash
    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    // Try invalid update
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

    // Verify hash hasn't changed
    test.assert_token_hash(token_id, &initial_hash)?;

    Ok(())
}

#[test]
fn hash_changes_correctly_after_update() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // Get initial hash
    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    // Perform valid update
    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result =
        test.ctx
            .tiles
            .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update.clone()])?;

    // Verify events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    // Calculate expected hash
    let mut expected_metadata = TileMetadata::default();
    expected_metadata.apply_updates(
        vec![update],
        &owner,
        test.ctx.app.inner().block_info().time.seconds(),
    );
    let expected_hash = expected_metadata.hash();

    // Verify hash has changed and matches expected
    test.assert_token_hash(token_id, &expected_hash)?;
    assert_ne!(
        initial_hash, expected_hash,
        "Hash should change after successful update"
    );

    Ok(())
}

#[test]
fn hash_matches_expected_after_multiple_updates() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // Get initial hash
    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    // Perform multiple updates
    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 99,
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result =
        test.ctx
            .tiles
            .update_pixel(&mut test.ctx.app, &owner, token_id, updates.clone())?;

    // Verify events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    // Calculate expected hash
    let mut expected_metadata = TileMetadata::default();
    expected_metadata.apply_updates(
        updates,
        &owner,
        test.ctx.app.inner().block_info().time.seconds(),
    );
    let expected_hash = expected_metadata.hash();

    // Verify hash matches expected
    test.assert_token_hash(token_id, &expected_hash)?;
    assert_ne!(
        initial_hash, expected_hash,
        "Hash should be different from initial"
    );

    Ok(())
}

#[test]
fn cannot_update_with_outdated_metadata() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // First update
    let update1 = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update1])?;
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    // Try second update with outdated metadata
    let update2 = PixelUpdate {
        id: 1,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };
    let result = test.ctx.tiles.update_pixel_with_metadata(
        &mut test.ctx.app,
        &owner,
        token_id,
        vec![update2],
        TileMetadata::default(), // Using default (outdated) metadata
    );
    test.assert_error_hash_mismatch(result);

    Ok(())
}

#[test]
fn cannot_update_non_expired_pixel() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // First update
    let initial_update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    let result = test.ctx.tiles.update_pixel(
        &mut test.ctx.app,
        &owner,
        token_id,
        vec![initial_update.clone()],
    )?;
    test.assert_pixel_update_event(&result, &token_id.to_string(), &owner);

    // Try to update the same pixel before expiration
    let update = PixelUpdate {
        id: 0,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };

    // Get current metadata to avoid hash mismatch error
    let mut current_metadata = TileMetadata::default();
    current_metadata.apply_updates(
        vec![initial_update],
        &owner,
        test.ctx.app.inner().block_info().time.seconds(),
    );

    let result = test.ctx.tiles.update_pixel_with_metadata(
        &mut test.ctx.app,
        &owner,
        token_id,
        vec![update],
        current_metadata,
    );

    test.assert_error_invalid_config(result, "Pixel is not expired yet");

    Ok(())
}

#[test]
fn payment_is_distributed_correctly_for_single_update() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600, // 1 hour
    };

    // Get initial balances
    let initial_owner_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&owner, "ustars")?
        .amount
        .u128();
    let initial_creator_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&test.ctx.users.get_creator().address, "ustars")?
        .amount
        .u128();

    // Calculate expected price (1 hour at base rate)
    let price_scaling = test.ctx.tiles.query_price_scaling(&test.ctx.app)?;
    let total_price = price_scaling.calculate_price(3600).u128();

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update])?;

    // Verify payment distribution
    test.assert_pixel_update_payment(&result, &token_id.to_string(), total_price);

    // Verify final balances
    let royalty_amount = total_price * 5 / 100; // 5% royalty
    let owner_amount = total_price - royalty_amount;

    // Owner pays total_price and receives owner_amount back
    test.assert_funds_received(
        &owner,
        initial_owner_balance - total_price + owner_amount,
        "ustars",
    );
    test.assert_funds_received(
        &test.ctx.users.get_creator().address,
        initial_creator_balance + royalty_amount,
        "ustars",
    );

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
            expiration_duration: 3600, // 1 hour
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration_duration: 86400, // 24 hours
        },
        PixelUpdate {
            id: 2,
            color: "#0000FF".to_string(),
            expiration_duration: 43200, // 12 hours
        },
    ];

    // Get initial balances
    let initial_owner_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&owner, "ustars")?
        .amount
        .u128();
    let initial_creator_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&test.ctx.users.get_creator().address, "ustars")?
        .amount
        .u128();

    // Calculate total price
    let price_scaling = test.ctx.tiles.query_price_scaling(&test.ctx.app)?;
    let total_price: u128 = updates
        .iter()
        .map(|update| {
            price_scaling
                .calculate_price(update.expiration_duration)
                .u128()
        })
        .sum();

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates)?;

    // Verify payment distribution
    test.assert_pixel_update_payment(&result, &token_id.to_string(), total_price);

    // Verify final balances
    let royalty_amount = total_price * 5 / 100; // 5% royalty
    let owner_amount = total_price - royalty_amount;

    // Owner pays total_price and receives owner_amount back
    test.assert_funds_received(
        &owner,
        initial_owner_balance - total_price + owner_amount,
        "ustars",
    );
    test.assert_funds_received(
        &test.ctx.users.get_creator().address,
        initial_creator_balance + royalty_amount,
        "ustars",
    );

    Ok(())
}

#[test]
fn payment_is_refunded_when_update_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

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
    ];

    // Get initial balance
    let initial_balance = test
        .ctx
        .app
        .inner()
        .wrap()
        .query_balance(&owner, "ustars")?
        .amount
        .u128();

    // Calculate total price
    let price_scaling = test.ctx.tiles.query_price_scaling(&test.ctx.app)?;
    let _total_price: u128 = updates
        .iter()
        .map(|update| {
            price_scaling
                .calculate_price(update.expiration_duration)
                .u128()
        })
        .sum();

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    // Verify update failed
    test.assert_error_invalid_config(result, "Invalid pixel id: 100");

    // Verify balance unchanged (payment refunded)
    test.assert_funds_received(&owner, initial_balance, "ustars");

    Ok(())
}
