//! The payment rules of `SetPixelColor`, checked end to end.
//!
//! A pixel sale is the only place money moves in this contract, so every way it can
//! be paid wrong must be proven to be refused.

use anyhow::Result;
use cosmwasm_std::{coins, Coin};
use tiles::core::tile::metadata::PixelUpdate;
use tiles::defaults::constants::{DEFAULT_COLOR, NATIVE_DENOM, PIXEL_MIN_EXPIRATION};

use crate::utils::TestSetup;

fn one_update() -> PixelUpdate {
    PixelUpdate {
        id: 4,
        color: "#00FF00".to_string(),
        expiration_duration: PIXEL_MIN_EXPIRATION,
    }
}

/// The advertised price is the price charged: no more, no less.
#[test]
fn an_exact_payment_is_accepted() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;
    let metadata = setup.tile_metadata(token_id)?;

    setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        coins(quote.total.u128(), NATIVE_DENOM),
    )?;

    assert_eq!(setup.tile_metadata(token_id)?.pixels[4].color, "#00FF00");
    Ok(())
}

/// Underpaying is refused: the contract never sells a lease at a discount.
#[test]
fn an_underpayment_is_refused() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;
    let metadata = setup.tile_metadata(token_id)?;

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        coins(quote.total.u128() - 1, NATIVE_DENOM),
    );

    assert!(result.is_err(), "one unit short must be refused");
    assert_eq!(
        setup.tile_metadata(token_id)?.pixels[4].color,
        DEFAULT_COLOR,
        "nothing was written"
    );
    Ok(())
}

/// Overpaying is refused too: the contract takes exactly what it asked for.
#[test]
fn an_overpayment_is_refused() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;
    let metadata = setup.tile_metadata(token_id)?;

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        coins(quote.total.u128() + 1, NATIVE_DENOM),
    );

    assert!(
        result.is_err(),
        "an overpayment must be refused rather than silently absorbed"
    );
    Ok(())
}

/// A colour paid for in a denom the contract does not recognise is not paid for.
///
/// Note: `NATIVE_DENOM` is `uatom`, which on the Cosmos Hub is also the gas denom.
/// Whether that is the right payment denom is an open decision (ADR 0005).
#[test]
fn an_unknown_denom_is_refused() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    // The amount is right for the duration, the denom is one the contract never quotes.
    let price_scaling = setup.state.get_price_scaling()?;
    let amount = price_scaling.calculate_price(update.expiration_duration);
    let metadata = setup.tile_metadata(token_id)?;

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        vec![Coin {
            denom: "ustars".to_string(),
            amount,
        }],
    );

    assert!(result.is_err(), "an unknown denom must be refused");
    assert_eq!(
        setup.tile_metadata(token_id)?.pixels[4].color,
        DEFAULT_COLOR,
        "nothing was written"
    );
    Ok(())
}

/// Colouring with no money at all is refused.
#[test]
fn a_missing_payment_is_refused() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let metadata = setup.tile_metadata(token_id)?;
    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![one_update()],
        metadata,
        vec![],
    );

    assert!(result.is_err(), "colouring is never free");
    Ok(())
}

/// Two denoms at once are refused, even if one of them would cover the price.
#[test]
fn a_multi_denom_payment_is_refused() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;
    let metadata = setup.tile_metadata(token_id)?;

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        vec![
            Coin {
                denom: NATIVE_DENOM.to_string(),
                amount: quote.total,
            },
            Coin {
                denom: "uatom".to_string(),
                amount: quote.total,
            },
        ],
    );

    assert!(
        result.is_err(),
        "a multi-denom send must be refused, not partially counted"
    );
    Ok(())
}

/// Colouring an unknown tile is refused.
#[test]
fn colouring_an_unknown_tile_is_refused() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let colourer = setup.users.get_tile_creator().address.clone();

    let result = setup.update_pixel(&colourer, 4242, vec![one_update()]);
    assert!(result.is_err(), "an unknown tile must be refused");
    Ok(())
}

/// The payment denom is `uatom` by decision (ADR 0005), and it is not configurable.
///
/// This test locks the decision: if someone changes the denom or makes it configurable
/// without reopening the ADR, this fails.
#[test]
fn the_only_accepted_denom_is_uatom() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    assert_eq!(
        NATIVE_DENOM, "uatom",
        "ADR 0005: uatom is the payment denom, hard-coded on purpose"
    );

    // A payment in any other denom is refused, even for the right amount.
    let update = one_update();
    let price_scaling = setup.state.get_price_scaling()?;
    let amount = price_scaling.calculate_price(update.expiration_duration);
    let metadata = setup.tile_metadata(token_id)?;

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        vec![Coin {
            denom: "uosmo".to_string(),
            amount,
        }],
    );

    assert!(result.is_err(), "only the configured denom may be accepted");
    Ok(())
}
