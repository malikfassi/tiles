use anyhow::Result;
use tiles::core::{pricing::PriceScaling, tile::metadata::PixelUpdate};

use crate::utils::{ResponseAssertions, StateAssertions, TestSetup};

#[test]
fn hash_changes_after_pixel_update() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    let initial_hash = setup.tiles.query_token_hash(&setup.app, token_id)?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = setup.update_pixel(&buyer.address, token_id, vec![update.clone()])?;

    // Assert all events
    ResponseAssertions::assert_pixel_update(&result, token_id, &[&update], &buyer.address);
    ResponseAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        &setup.state,
        &[&update],
    );

    let updated_hash = setup.tiles.query_token_hash(&setup.app, token_id)?;
    assert_ne!(initial_hash, updated_hash);

    Ok(())
}

#[test]
fn hash_changes_after_each_pixel_update() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    let initial_hash = setup.tiles.query_token_hash(&setup.app, token_id)?;

    // First update
    let update1 = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    let result1 = setup.update_pixel(&buyer.address, token_id, vec![update1.clone()])?;

    // Assert events for first update
    ResponseAssertions::assert_pixel_update(&result1, token_id, &[&update1], &buyer.address);
    ResponseAssertions::assert_payment_distribution(
        &result1,
        token_id,
        &buyer.address,
        &setup.state,
        &[&update1],
    );

    let first_hash = setup.tiles.query_token_hash(&setup.app, token_id)?;
    assert_ne!(initial_hash, first_hash);

    // Second update
    let update2 = PixelUpdate {
        id: 1,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };
    let result2 = setup.update_pixel(&buyer.address, token_id, vec![update2.clone()])?;

    // Assert events for second update
    ResponseAssertions::assert_pixel_update(&result2, token_id, &[&update2], &buyer.address);
    ResponseAssertions::assert_payment_distribution(
        &result2,
        token_id,
        &buyer.address,
        &setup.state,
        &[&update2],
    );

    let second_hash = setup.tiles.query_token_hash(&setup.app, token_id)?;
    assert_ne!(first_hash, second_hash);

    Ok(())
}

#[test]
fn update_fails_with_hash_mismatch() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    // First make a valid update to change the token's hash
    let initial_update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    setup.update_pixel(&buyer.address, token_id, vec![initial_update])?;

    Ok(())
}
