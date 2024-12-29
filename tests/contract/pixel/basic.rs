use anyhow::Result;
use tiles::{
    core::pricing::PriceScaling, core::tile::metadata::PixelUpdate,
    defaults::constants::DEFAULT_ROYALTY_SHARE,
};

use crate::common::{EventAssertions, TestContext};

#[test]
fn can_set_pixel_color() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;

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

    Ok(())
}

#[test]
fn all_valid_updates_succeed() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;

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

    let result = ctx.update_pixel(&buyer.address, token_id, updates.clone())?;

    // Calculate total price using PriceScaling
    let price_scaling = PriceScaling::default();
    let total_price = price_scaling
        .calculate_total_price(updates.iter().map(|u| &u.expiration_duration))
        .u128();
    let royalty_amount = total_price * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount = total_price - royalty_amount;

    // Assert all events for each update
    EventAssertions::assert_pixel_update(&result, token_id, &updates, &buyer.address);
    EventAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        royalty_amount,
        owner_amount,
    );

    Ok(())
}

#[test]
fn can_update_multiple_pixels() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;

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

    let result = ctx.update_pixel(&buyer.address, token_id, updates.clone())?;

    // Calculate total price using PriceScaling
    let price_scaling = PriceScaling::default();
    let total_price = price_scaling
        .calculate_total_price(updates.iter().map(|u| &u.expiration_duration))
        .u128();
    let royalty_amount = total_price * DEFAULT_ROYALTY_SHARE as u128 / 100;
    let owner_amount = total_price - royalty_amount;

    // Assert all events for each update
    EventAssertions::assert_pixel_update(&result, token_id, &updates, &buyer.address);
    EventAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        royalty_amount,
        owner_amount,
    );

    Ok(())
}
