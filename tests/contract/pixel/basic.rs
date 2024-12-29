use anyhow::Result;
use tiles::{
    core::pricing::PriceScaling, core::tile::metadata::PixelUpdate,
    defaults::constants::DEFAULT_ROYALTY_SHARE,
};

use crate::utils::{ResponseAssertions, StateAssertions, TestSetup};

#[test]
fn can_set_pixel_color() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = setup.update_pixel(&buyer.address, token_id, vec![update.clone()])?;

    ResponseAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        &setup.state,
        &[&update],
    );

    Ok(())
}

#[test]
fn all_valid_updates_succeed() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

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

    let result = setup.update_pixel(&buyer.address, token_id, updates.clone())?;

    ResponseAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        &setup.state,
        &updates.iter().collect::<Vec<_>>(),
    );

    Ok(())
}

#[test]
fn can_update_multiple_pixels() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

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

    let result = setup.update_pixel(&buyer.address, token_id, updates.clone())?;

    ResponseAssertions::assert_payment_distribution(
        &result,
        token_id,
        &buyer.address,
        &setup.state,
        &updates.iter().collect::<Vec<_>>(),
    );

    Ok(())
}
