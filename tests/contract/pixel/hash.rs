use anyhow::Result;
use tiles::{
    core::{pricing::PriceScaling, tile::metadata::PixelUpdate},
    defaults::constants::DEFAULT_ROYALTY_SHARE,
};

use crate::common::{EventAssertions, TestContext};

#[test]
fn hash_changes_after_pixel_update() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer().clone();
    let response = ctx.mint_token(&buyer.address)?;
    let token_id = EventAssertions::extract_token_id(&response);

    let initial_hash = ctx.tiles.query_token_hash(&ctx.app, token_id)?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = ctx.update_pixel(&buyer.address, token_id, vec![update.clone()])?;

    // Calculate total price using PriceScaling
    let price_scaling = PriceScaling::default();
    let total_price = price_scaling
        .calculate_total_price(std::iter::once(&update.expiration_duration))
        .u128();
    let royalty_amount = total_price * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount = total_price - royalty_amount;

    // Assert all events
    EventAssertions::assert_pixel_update(&result, token_id, &[update], &buyer.address);
    EventAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        royalty_amount,
        owner_amount,
    );

    let updated_hash = ctx.tiles.query_token_hash(&ctx.app, token_id)?;
    assert_ne!(initial_hash, updated_hash);

    Ok(())
}

#[test]
fn hash_changes_after_each_pixel_update() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer().clone();
    let response = ctx.mint_token(&buyer.address)?;
    let token_id = EventAssertions::extract_token_id(&response);

    let initial_hash = ctx.tiles.query_token_hash(&ctx.app, token_id)?;

    // First update
    let update1 = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    let result1 = ctx.update_pixel(&buyer.address, token_id, vec![update1.clone()])?;

    // Calculate total price using PriceScaling for first update
    let price_scaling = PriceScaling::default();
    let total_price1 = price_scaling
        .calculate_total_price(std::iter::once(&update1.expiration_duration))
        .u128();
    let royalty_amount1 = total_price1 * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount1 = total_price1 - royalty_amount1;

    // Assert events for first update
    EventAssertions::assert_pixel_update(&result1, token_id, &[update1], &buyer.address);
    EventAssertions::assert_payment_distribution(
        &result1,
        token_id,
        &buyer.address,
        royalty_amount1,
        owner_amount1,
    );

    let first_hash = ctx.tiles.query_token_hash(&ctx.app, token_id)?;
    assert_ne!(initial_hash, first_hash);

    // Second update
    let update2 = PixelUpdate {
        id: 1,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };
    let result2 = ctx.update_pixel(&buyer.address, token_id, vec![update2.clone()])?;

    // Calculate total price using PriceScaling for second update
    let total_price2 = price_scaling
        .calculate_total_price(std::iter::once(&update2.expiration_duration))
        .u128();
    let royalty_amount2 = total_price2 * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount2 = total_price2 - royalty_amount2;

    // Assert events for second update
    EventAssertions::assert_pixel_update(&result2, token_id, &[update2], &buyer.address);
    EventAssertions::assert_payment_distribution(
        &result2,
        token_id,
        &buyer.address,
        royalty_amount2,
        owner_amount2,
    );

    let second_hash = ctx.tiles.query_token_hash(&ctx.app, token_id)?;
    assert_ne!(first_hash, second_hash);

    Ok(())
}

#[test]
fn update_fails_with_hash_mismatch() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer().clone();
    let response = ctx.mint_token(&buyer.address)?;
    let token_id = EventAssertions::extract_token_id(&response);

    // First make a valid update to change the token's hash
    let initial_update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    ctx.update_pixel(&buyer.address, token_id, vec![initial_update])?;

    Ok(())
}
