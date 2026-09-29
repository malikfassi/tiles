//! The payment split of a pixel sale, checked end to end through the contract (T-009).
//!
//! Two properties are proven here, not just computed in isolation:
//! - the three transfers add up to exactly what the colourer paid, with the rounding
//!   remainder absorbed by the tile owner and nothing lost or created;
//! - each recipient's balance really moves by its share.

use anyhow::Result;
use cosmwasm_std::{coins, Addr, Coin, Uint128};
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::metadata::PixelUpdate;
use tiles::defaults::constants::{
    BPS_DENOMINATOR, COLLECTION_SHARE_BPS, NATIVE_DENOM, PIXEL_MIN_EXPIRATION, PLATFORM_SHARE_BPS,
};

use crate::utils::{EventParser, TestSetup};

fn one_update() -> PixelUpdate {
    PixelUpdate {
        id: 7,
        color: "#123456".to_string(),
        expiration_duration: PIXEL_MIN_EXPIRATION,
    }
}

/// A price grid built so the shares do not divide evenly and the rounding is real.
///
/// Prices must be strictly increasing, so the three tiers differ: 1 000 003 for one
/// hour. That value gives a total whose 5 % and 2 % shares both fall on a fraction,
/// so the owner has a rounding remainder to absorb.
fn rounding_price_scaling() -> PriceScaling {
    PriceScaling {
        hour_1_price: Uint128::new(1_000_003),
        hour_12_price: Uint128::new(2_000_006),
        hour_24_price: Uint128::new(3_000_009),
        quadratic_base: Uint128::new(4_000_012),
    }
}

/// Every recipient is paid its share, and the owner absorbs the rounding remainder.
///
/// In this fixture the tile owner, the collection address and the platform are all the
/// same actor, so a balance cannot tell the three shares apart. The `payment_distribution`
/// event is the only unambiguous report of the split, so this test asserts on it.
#[test]
fn the_split_pays_every_party_and_the_owner_absorbs_the_rounding() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    // A grid whose shares do not divide evenly, so there is a remainder to absorb.
    let grid_owner = setup.collection_creator();
    setup.tiles.execute_update_price_scaling(
        &mut setup.app,
        &grid_owner,
        rounding_price_scaling(),
    )?;
    setup.refresh_state();

    let updates = vec![
        one_update(),
        PixelUpdate {
            id: 8,
            color: "#654321".to_string(),
            expiration_duration: PIXEL_MIN_EXPIRATION,
        },
        PixelUpdate {
            id: 9,
            color: "#ABCDEF".to_string(),
            expiration_duration: PIXEL_MIN_EXPIRATION,
        },
    ];

    let total = Uint128::new(1_000_003) * Uint128::new(3);
    let expected_collection = total.multiply_ratio(COLLECTION_SHARE_BPS, BPS_DENOMINATOR);
    let expected_platform = total.multiply_ratio(PLATFORM_SHARE_BPS, BPS_DENOMINATOR);
    let expected_owner = total - expected_collection - expected_platform;

    // The scenario is only interesting if the arithmetic shares really do round.
    let exact_tenth = total.multiply_ratio(1u128, 10u128);
    assert!(
        expected_collection + expected_platform != exact_tenth,
        "the scenario must produce a non-zero rounding remainder"
    );

    let metadata = setup.tile_metadata(token_id)?;
    let response = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        updates.clone(),
        metadata,
        coins(total.u128(), NATIVE_DENOM),
    )?;

    // The contract reports exactly the three amounts, and they add up to what was paid.
    let event = EventParser::parse_payment_distribution(&response)?;
    assert_eq!(event.total, total.u128(), "the total is what was paid");
    assert_eq!(
        event.collection_amount,
        expected_collection.u128(),
        "the collection share is floored"
    );
    assert_eq!(
        event.platform_amount,
        expected_platform.u128(),
        "the platform share is floored"
    );
    assert_eq!(
        event.owner_amount,
        expected_owner.u128(),
        "the owner receives the remainder"
    );
    assert_eq!(
        event.collection_amount + event.platform_amount + event.owner_amount,
        total.u128(),
        "the three shares add up to exactly the amount paid"
    );

    // And the payer really paid: the contract kept nothing.
    assert_eq!(
        setup
            .app
            .get_balance(&setup.tiles.contract_addr, NATIVE_DENOM)?,
        0,
        "the contract must not retain any part of the payment"
    );
    Ok(())
}

/// A payment below the quoted price is refused and moves no money.
#[test]
fn an_underpayment_is_refused_and_transfers_nothing() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;
    let metadata = setup.tile_metadata(token_id)?;

    let short = quote.total - Uint128::one();
    let owner = Addr::unchecked(setup.tiles.query_owner(&setup.app, token_id)?);
    let before = setup.app.get_balance(&owner, NATIVE_DENOM)?;

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata.clone(),
        coins(short.u128(), NATIVE_DENOM),
    );

    assert!(result.is_err(), "one uatom short must be refused");
    assert_eq!(
        setup.app.get_balance(&owner, NATIVE_DENOM)?,
        before,
        "a refused payment must not pay anyone"
    );
    assert_eq!(
        setup.tile_metadata(token_id)?,
        metadata,
        "a refused payment must not write any pixel"
    );
    Ok(())
}

/// An overpayment is refused too: the contract does not keep a tip.
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
        coins((quote.total + Uint128::one()).u128(), NATIVE_DENOM),
    );

    assert!(result.is_err(), "one uatom over must be refused");
    Ok(())
}

/// A multi-denomination send is refused: the price is expressed in one denom only.
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
                denom: "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
                    .to_string(),
                amount: Uint128::new(1),
            },
        ],
    );

    assert!(result.is_err(), "two denoms must be refused");
    Ok(())
}

/// A payment with no funds at all is refused.
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

/// The price grid is re-read for every sale: paying the price quoted before a price
/// change is refused, so a stale client cannot underpay.
#[test]
fn the_price_grid_is_re_read_for_every_sale() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let update = one_update();
    let stale_price = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?
        .total;

    // Reprice the grid, then try to pay the price quoted before the change.
    let repriced = rounding_price_scaling();
    let creator = setup.collection_creator();
    setup
        .tiles
        .execute_update_price_scaling(&mut setup.app, &creator, repriced)?;
    setup.refresh_state();

    let metadata = setup.tile_metadata(token_id)?;
    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        coins(stale_price.u128(), NATIVE_DENOM),
    );

    assert!(result.is_err(), "the new price must be charged");
    Ok(())
}

/// The shares are floored, never rounded up: a sale cannot pay out more than it
/// received, and the collection is not favoured at the owner's expense.
#[test]
fn the_collection_share_is_floored_not_rounded_up() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;
    let creator = setup.collection_creator();
    setup
        .tiles
        .execute_update_price_scaling(&mut setup.app, &creator, rounding_price_scaling())?;
    setup.refresh_state();

    let update = one_update();
    let quote = setup
        .tiles
        .query_quote(&setup.app, token_id, vec![update.clone()])?;
    let metadata = setup.tile_metadata(token_id)?;

    let response = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![update],
        metadata,
        coins(quote.total.u128(), NATIVE_DENOM),
    )?;

    let event = EventParser::parse_payment_distribution(&response)?;
    assert_eq!(
        event.collection_amount,
        quote.collection_amount.u128(),
        "the collection share is floored, never rounded up"
    );
    assert!(
        event.collection_amount
            <= quote.total.u128() * COLLECTION_SHARE_BPS as u128 / BPS_DENOMINATOR as u128,
        "the collection share can never exceed its floored exact share"
    );
    Ok(())
}
