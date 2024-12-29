use anyhow::Result;
use tiles::{
    core::pricing::PriceScaling, core::tile::metadata::PixelUpdate,
    defaults::constants::DEFAULT_ROYALTY_SHARE,
};

use crate::common::{EventAssertions, TestContext};

#[test]
fn test_payment_distribution() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer().clone();
    let response = ctx.mint_token(&buyer.address)?;
    let token_id = EventAssertions::extract_token_id(&response);

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
