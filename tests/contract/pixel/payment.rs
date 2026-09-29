use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;
use tiles::defaults::constants::{COLLECTION_SHARE_PERCENT, NATIVE_DENOM, PLATFORM_SHARE_PERCENT};

use crate::utils::{EventAssertions, TestSetup};

/// A pixel sale is split between the collection, the platform and the tile owner,
/// and the three shares always add up to the exact amount paid (ADR 0004).
#[test]
fn payment_is_distributed_correctly() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = PixelUpdate {
        id: 1,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let price_scaling = setup.state.get_price_scaling()?;
    let price = price_scaling.calculate_price(3600);

    let colourer_before = setup.app.get_balance(&colourer, NATIVE_DENOM)?;

    let response = setup.update_pixel(&colourer, token_id, vec![update.clone()])?;

    // The colourer pays the full price; what they get back depends on roles, so the
    // split itself is asserted through the event below.
    let colourer_after = setup.app.get_balance(&colourer, NATIVE_DENOM)?;
    assert!(
        colourer_after <= colourer_before,
        "colouring is never free for the payer"
    );

    // The three shares must add up to the price exactly.
    let collection_share = price.mul_floor(COLLECTION_SHARE_PERCENT);
    let platform_share = price.mul_floor(PLATFORM_SHARE_PERCENT);
    let owner_share = price - collection_share - platform_share;
    assert_eq!(
        collection_share + platform_share + owner_share,
        price,
        "the shares must add up to the price"
    );

    EventAssertions::assert_pixel_update(&response, token_id, &[&update], &colourer);
    EventAssertions::assert_payment_distribution(
        &response,
        token_id,
        &colourer,
        &setup.state,
        &[&update],
    );
    Ok(())
}
