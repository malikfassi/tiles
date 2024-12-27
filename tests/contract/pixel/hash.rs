use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};
use tiles::defaults::constants::DEFAULT_ROYALTY_SHARE;

#[test]
fn hash_changes_after_pixel_update() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

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

    let updated_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_ne!(initial_hash, updated_hash);

    Ok(())
}

#[test]
fn hash_changes_after_each_pixel_update() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    // First update
    let update1 = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    let result1 = test.update_pixels(token_id, vec![update1.clone()], &owner)?;

    // Assert events for first update
    test.assert_pixel_update_event(&result1, &token_id.to_string(), &update1, &owner);
    test.assert_payment_distribution_event(
        &result1,
        &token_id.to_string(),
        &owner,
        100000000 * DEFAULT_ROYALTY_SHARE as u128 / 100, // royalty amount
        100000000 * (100 - DEFAULT_ROYALTY_SHARE) as u128 / 100, // owner amount
    );

    let first_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_ne!(initial_hash, first_hash);

    // Second update
    let update2 = PixelUpdate {
        id: 1,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };
    let result2 = test.update_pixels(token_id, vec![update2.clone()], &owner)?;

    // Assert events for second update
    test.assert_pixel_update_event(&result2, &token_id.to_string(), &update2, &owner);
    test.assert_payment_distribution_event(
        &result2,
        &token_id.to_string(),
        &owner,
        100000000 * DEFAULT_ROYALTY_SHARE as u128 / 100, // royalty amount
        100000000 * (100 - DEFAULT_ROYALTY_SHARE) as u128 / 100, // owner amount
    );

    let second_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_ne!(first_hash, second_hash);

    Ok(())
}

#[test]
fn update_fails_with_hash_mismatch() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // First make a valid update to change the token's hash
    let initial_update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    test.update_pixels(token_id, vec![initial_update], &owner)?;

    // Now try to update with default metadata (which will have wrong hash)
    let update = PixelUpdate {
        id: 1,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };

    let result = test.ctx.tiles.update_pixel(
        &mut test.ctx.app,
        &owner,
        token_id,
        vec![update],
        TileMetadata::default(), // This will have wrong hash since we modified pixel 0
    );

    test.assert_error_hash_mismatch(result);
    Ok(())
}
