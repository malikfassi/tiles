use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

use crate::utils::TestSetup;

/// A quote is a promise: what the contract advertises must be exactly what it charges.
#[test]
fn quote_matches_the_price_actually_charged() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = PixelUpdate {
        id: 3,
        color: "#123456".to_string(),
        expiration_duration: 3600,
    };

    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;

    // Same split rule as the execute path, checked on the returned amounts.
    assert_eq!(
        quote.collection_amount + quote.platform_amount + quote.owner_amount,
        quote.total,
        "the quoted shares must add up to the quoted total"
    );

    // Paying exactly the quote must succeed.
    let metadata = setup.tile_metadata(token_id)?;
    setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        cosmwasm_std::coins(quote.total.u128(), tiles::defaults::constants::NATIVE_DENOM),
    )?;

    let after = setup.tile_metadata(token_id)?;
    assert_eq!(after.pixels[3].color, "#123456", "the pixel was coloured");
    Ok(())
}

/// The quote is also a guard: undefined writes are refused before anything is paid.
#[test]
fn quote_refuses_an_invalid_update() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let bad_colour = PixelUpdate {
        id: 0,
        color: "not-a-colour".to_string(),
        expiration_duration: 3600,
    };

    let result = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![bad_colour]);
    assert!(result.is_err(), "an invalid colour must not be quoted");
    Ok(())
}
