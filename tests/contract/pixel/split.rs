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
    BPS_DENOMINATOR, COLLECTION_SHARE_BPS, NATIVE_DENOM, PLATFORM_SHARE_BPS, PIXEL_MIN_EXPIRATION,
};

use crate::utils::{EventAssertions, TestSetup};

fn one_update() -> PixelUpdate {
    PixelUpdate {
        id: 7,
        color: "#123456".to_string(),
        expiration_duration: PIXEL_MIN_EXPIRATION,
    }
}

/// A price grid built so the shares do not divide evenly and the rounding is real.
///
/// 3 pixels at 1 000 003 uatom each give a total of 3 000 009: 5 % is 150 000.45 and
/// 2 % is 60 000.18, so both floors lose a fraction that the owner must absorb.
fn rounding_price_scaling() -> PriceScaling {
    PriceScaling {
        hour_1_price: Uint128::new(1_000_003),
        hour_12_price: Uint128::new(1_000_003),
        hour_24_price: Uint128::new(1_000_003),
        quadratic_base: Uint128::zero(),
    }
}

/// Two snapshots of a set of balances, so a test can assert the exact deltas.
struct Balances(Vec<(Addr, Uint128)>);

impl Balances {
    fn snapshot(setup: &TestSetup, addresses: &[&Addr]) -> Result<Self> {
        let mut balances = Vec::with_capacity(addresses.len());
        for address in addresses {
            balances.push((
                (*address).clone(),
                setup.app.get_balance(address, NATIVE_DENOM)?,
            ));
        }
        Ok(Self(balances))
    }

    fn delta(&self, address: &Addr, before: &Self) -> Result<Uint128> {
        let now = self.balance(address)?;
        let then = before.balance(address)?;
        Ok(now.checked_sub(then)?)
    }

    fn balance(&self, address: &Addr) -> Result<Uint128> {
        self.0
            .iter()
            .find(|(addr, _)| addr == address)
            .map(|(_, balance)| *balance)
            .ok_or_else(|| anyhow::anyhow!("balance not snapshotted for {}", address))
    }
}

/// Every recipient is paid its share, and the owner absorbs the rounding remainder.
#[test]
fn the_split_pays_every_party_and_the_owner_absorbs_the_rounding() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let colourer = setup.users.get_tile_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;
    let creator = setup.collection_creator();
    setup
        .tiles
        .execute_update_price_scaling(&mut setup.app, &creator, rounding_price_scaling())?;
    setup.refresh_state();

    let owner = setup.tiles.query_owner_of(&setup.app, token_id)?.address;
    let collection = setup.tiles.query_collection_payment_address(&setup.app)?;
    let platform = setup.tiles.query_platform_payment_address(&setup.app)?;

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

    // The remainder is what the owner absorbs: the exact arithmetic share is fractional.
    let exact_owner = total.multiply_ratio(
        BPS_DENOMINATOR - COLLECTION_SHARE_BPS - PLATFORM_SHARE_BPS,
        BPS_DENOMINATOR,
    );
    assert!(
        expected_owner > exact_owner,
        "the scenario must produce a non-zero rounding remainder"
    );

    let before = Balances::snapshot(&setup, &[&owner, &collection, &platform])?;
    let metadata = setup.tile_metadata(token_id)?;
    let response = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        updates.clone(),
        metadata,
        coins(total.u128(), NATIVE_DENOM),
    )?;
    let after = Balances::snapshot(&setup, &[&owner, &collection, &platform])?;

    // The three parts add up to exactly what was paid.
    assert_eq!(
        expected_collection + expected_platform + expected_owner,
        total,
        "the shares must add up to the amount paid"
    );

    // And the balances actually moved by those amounts.
    assert_eq!(after.delta(&collection, &before)?, expected_collection);
    assert_eq!(after.delta(&platform, &before)?, expected_platform);
    assert_eq!(after.delta(&owner, &before)?, expected_owner);

    // The event an indexer reads reports the same three numbers.
    EventAssertions::assert_payment_distribution(
        &response,
        token_id,
        &colourer,
        &setup.state,
        &updates,
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
    let owner = setup.tiles.query_owner_of(&setup.app, token_id)?.address;
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

    let collection = setup.tiles.query_collection_payment_address(&setup.app)?;
    let before = setup.app.get_balance(&collection, NATIVE_DENOM)?;

    let metadata = setup.tile_metadata(token_id)?;
    setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &colourer,
        token_id,
        vec![one_update()],
        metadata,
        coins(1_000_003, NATIVE_DENOM),
    )?;

    let collected = setup.app.get_balance(&collection, NATIVE_DENOM)? - before;
    assert_eq!(
        collected,
        Uint128::new(1_000_003).multiply_ratio(COLLECTION_SHARE_BPS, BPS_DENOMINATOR),
        "the collection share is floored, never rounded up"
    );
    Ok(())
}
