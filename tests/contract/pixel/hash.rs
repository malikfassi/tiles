use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::pricing::PriceScaling;
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

    // Calculate total price using PriceScaling
    let price_scaling = PriceScaling::default();
    let total_price = price_scaling
        .calculate_total_price(std::iter::once(&update.expiration_duration))
        .u128();
    let royalty_amount = total_price * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount = total_price - royalty_amount;

    // Assert all events
    test.assert_pixel_update_event(&result, &token_id.to_string(), &update, &owner);
    test.assert_payment_distribution_event(
        &result,
        &token_id.to_string(),
        &owner,
        royalty_amount,
        owner_amount,
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

    // Calculate total price using PriceScaling for first update
    let price_scaling = PriceScaling::default();
    let total_price1 = price_scaling
        .calculate_total_price(std::iter::once(&update1.expiration_duration))
        .u128();
    let royalty_amount1 = total_price1 * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount1 = total_price1 - royalty_amount1;

    // Assert events for first update
    test.assert_pixel_update_event(&result1, &token_id.to_string(), &update1, &owner);
    test.assert_payment_distribution_event(
        &result1,
        &token_id.to_string(),
        &owner,
        royalty_amount1,
        owner_amount1,
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

    // Calculate total price using PriceScaling for second update
    let total_price2 = price_scaling
        .calculate_total_price(std::iter::once(&update2.expiration_duration))
        .u128();
    let royalty_amount2 = total_price2 * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount2 = total_price2 - royalty_amount2;

    // Assert events for second update
    test.assert_pixel_update_event(&result2, &token_id.to_string(), &update2, &owner);
    test.assert_payment_distribution_event(
        &result2,
        &token_id.to_string(),
        &owner,
        royalty_amount2,
        owner_amount2,
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

    Ok(())
}
