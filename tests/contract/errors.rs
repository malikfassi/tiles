//! One test per `ContractError` variant, asserting the variant, not just that it failed (T-010).
//!
//! `cw-multi-test` wraps a contract failure in a text report that does **not** carry the
//! contract's own error, so a variant can only be asserted by calling the handlers
//! directly with mocked dependencies. That is what this file does: the point is to pin
//! *which* error comes back, because a test asserting only `is_err()` would pass even if
//! the contract reported `TileNotFound` for a tile that exists.

use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier};
use cosmwasm_std::{Addr, Coin, Decimal, DepsMut, MemoryStorage, OwnedDeps, Response};
use cw721::msg::{CollectionExtensionMsg, RoyaltyInfoResponse};
use tiles::contract::error::ContractError;
use tiles::contract::execute::execute_handler;
use tiles::contract::instantiate::{instantiate_handler, InstantiateMsg};
use tiles::contract::msg::{ExecuteMsg, TileExecuteMsg};
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};
use tiles::defaults::constants::{NATIVE_DENOM, PIXEL_MIN_EXPIRATION};

type Deps = OwnedDeps<MemoryStorage, MockApi, MockQuerier>;

/// The instantiated contract, with the actors that exercise it.
struct Fixture {
    deps: Deps,
    creator: Addr,
    colourer: Addr,
}

/// A valid `cosmwasm1...` address for a label, matching what the mock chain accepts.
fn account(label: &str) -> Addr {
    MockApi::default().addr_make(label)
}

/// Instantiates the contract with mocked dependencies.
fn fixture() -> Fixture {
    let mut deps = mock_dependencies();
    let creator = account("creator");

    let collection_info: CollectionExtensionMsg<RoyaltyInfoResponse> = CollectionExtensionMsg {
        description: Some("Tiles".to_string()),
        image: Some("ipfs://collection-image".to_string()),
        external_link: None,
        banner_url: None,
        explicit_content: None,
        start_trading_time: None,
        royalty_info: Some(RoyaltyInfoResponse {
            payment_address: creator.to_string(),
            share: Decimal::percent(5),
        }),
    };

    let msg = InstantiateMsg {
        name: "Tiles".to_string(),
        symbol: "TILE".to_string(),
        collection_info_extension: Some(collection_info),
        minter: None,
        creator: Some(creator.to_string()),
        withdraw_address: None,
    };

    instantiate_handler(deps.as_mut(), mock_env(), message_info(&creator, &[]), msg)
        .expect("instantiation succeeds");

    Fixture {
        deps,
        colourer: account("colourer"),
        creator,
    }
}

/// Mints tile "1" with the hash that `set_pixel` sends, so payment rules are reachable.
fn minted(f: &mut Fixture) {
    execute_handler(
        f.deps.as_mut(),
        mock_env(),
        message_info(&f.creator.clone(), &[]),
        ExecuteMsg::Mint {
            token_id: "1".to_string(),
            owner: f.creator.to_string(),
            token_uri: None,
            extension: tiles::core::Tile {
                tile_hash: TileMetadata::default().hash(),
                metadata: TileMetadata::default(),
            },
        },
    )
    .expect("minting succeeds");
}

/// Sends `SetPixelColor` on a tile that does not exist, so the message shape is valid.
///
/// Validation of the message happens before the tile is loaded, so the shapes of the
/// funds and the updates are what each test varies.
fn set_pixel(
    deps: DepsMut,
    sender: &Addr,
    token_id: &str,
    updates: Vec<PixelUpdate>,
    funds: &[Coin],
) -> Result<Response, ContractError> {
    execute_handler(
        deps,
        mock_env(),
        message_info(sender, funds),
        ExecuteMsg::UpdateExtension {
            msg: TileExecuteMsg::SetPixelColor {
                token_id: token_id.to_string(),
                current_metadata: TileMetadata::default(),
                updates,
            },
        },
    )
}

/// Sends `UpdatePriceScaling`, the only other message that can fail.
fn set_grid(
    deps: DepsMut,
    sender: &Addr,
    scaling: PriceScaling,
) -> Result<Response, ContractError> {
    execute_handler(
        deps,
        mock_env(),
        message_info(sender, &[]),
        ExecuteMsg::UpdateExtension {
            msg: TileExecuteMsg::UpdatePriceScaling(Box::new(scaling)),
        },
    )
}

fn update(id: u8, color: &str, duration: u64) -> PixelUpdate {
    PixelUpdate {
        id,
        color: color.to_string(),
        expiration_duration: duration,
    }
}

/// Asserts the exact variant, so a different error cannot pass the test.
#[track_caller]
fn assert_error(result: Result<Response, ContractError>, expected: ContractError) {
    match result {
        Err(actual) => assert_eq!(actual, expected, "wrong error variant"),
        Ok(_) => panic!("expected {expected}, but the call succeeded"),
    }
}

// ---- Message validation ----

/// `InvalidPixelId`: one past the last pixel of a 100-pixel tile.
#[test]
fn an_out_of_bounds_pixel_id_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(1_000_000u128, NATIVE_DENOM)];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(100, "#FF0000", PIXEL_MIN_EXPIRATION)],
            &funds,
        ),
        ContractError::InvalidPixelId { id: 100 },
    );
}

/// `InvalidColorFormat`: the colour is not `#RRGGBB`.
#[test]
fn a_malformed_colour_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(1_000_000u128, NATIVE_DENOM)];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "red", PIXEL_MIN_EXPIRATION)],
            &funds,
        ),
        ContractError::InvalidColorFormat {},
    );
}

/// `DuplicatePixelId`: the same pixel twice in one message.
#[test]
fn a_duplicated_pixel_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(1_000_000u128, NATIVE_DENOM)];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![
                update(3, "#FF0000", PIXEL_MIN_EXPIRATION),
                update(3, "#00FF00", PIXEL_MIN_EXPIRATION),
            ],
            &funds,
        ),
        ContractError::DuplicatePixelId { id: 3 },
    );
}

/// `InvalidLeaseTooShort`: below the one-hour floor.
#[test]
fn a_lease_below_the_minimum_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(1_000_000u128, NATIVE_DENOM)];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "#FF0000", PIXEL_MIN_EXPIRATION - 1)],
            &funds,
        ),
        ContractError::InvalidLeaseTooShort {
            duration: PIXEL_MIN_EXPIRATION - 1,
            min: PIXEL_MIN_EXPIRATION,
        },
    );
}

/// `InvalidLeaseTooLong`: above the twenty-four hour ceiling.
#[test]
fn a_lease_above_the_maximum_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(1_000_000u128, NATIVE_DENOM)];
    let too_long = tiles::defaults::constants::PIXEL_MAX_EXPIRATION + 1;

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "#FF0000", too_long)],
            &funds,
        ),
        ContractError::InvalidLeaseTooLong {
            duration: too_long,
            max: tiles::defaults::constants::PIXEL_MAX_EXPIRATION,
        },
    );
}

/// `EmptyUpdates`: nothing to colour.
#[test]
fn an_empty_update_list_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(1_000_000u128, NATIVE_DENOM)];

    assert_error(
        set_pixel(f.deps.as_mut(), &f.colourer.clone(), "1", vec![], &funds),
        ContractError::EmptyUpdates {},
    );
}

// ---- State and business rules ----

/// `TileNotFound`: the token id was never minted.
///
/// This also pins the order of the checks: an unknown tile is reported before the
/// payment is examined, so a caller that pays the right amount still gets this error.
#[test]
fn an_unknown_tile_is_reported() {
    let mut f = fixture();
    let funds = vec![Coin::new(
        tiles::defaults::constants::DEFAULT_PRICE_1_HOUR,
        NATIVE_DENOM,
    )];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "does-not-exist",
            vec![update(0, "#FF0000", PIXEL_MIN_EXPIRATION)],
            &funds,
        ),
        ContractError::TileNotFound {
            token_id: "does-not-exist".to_string(),
        },
    );
}

/// `MetadataHashMismatch`: the caller sent a `current_metadata` that is not the stored one.
///
/// The payment is checked first, so the exact price is required to reach the lock: this
/// test therefore also pins the order of the checks.
#[test]
fn a_stale_metadata_hash_is_reported() {
    let mut f = fixture();
    minted(&mut f);

    // The caller believes pixel 0 is red; the stored tile still has it white. The
    // optimistic lock must refuse the write rather than overwrite a fresher state.
    let mut stale = TileMetadata::default();
    stale.pixels[0].color = "#FF0000".to_string();

    let price = tiles::defaults::constants::DEFAULT_PRICE_1_HOUR;
    let funds = vec![Coin::new(price, NATIVE_DENOM)];

    assert_error(
        execute_handler(
            f.deps.as_mut(),
            mock_env(),
            message_info(&f.colourer.clone(), &funds),
            ExecuteMsg::UpdateExtension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: "1".to_string(),
                    current_metadata: stale,
                    updates: vec![update(0, "#00FF00", PIXEL_MIN_EXPIRATION)],
                },
            },
        ),
        ContractError::MetadataHashMismatch {},
    );
}

/// `Unauthorized`: only the collection payment address may reprice the grid.
#[test]
fn a_stranger_cannot_reprice_the_grid() {
    let mut f = fixture();

    assert_error(
        set_grid(
            f.deps.as_mut(),
            &f.colourer.clone(),
            PriceScaling {
                hour_1_price: tiles::defaults::constants::DEFAULT_PRICE_1_HOUR.into(),
                hour_12_price: tiles::defaults::constants::DEFAULT_PRICE_12_HOURS.into(),
                hour_24_price: tiles::defaults::constants::DEFAULT_PRICE_24_HOURS.into(),
                quadratic_base: tiles::defaults::constants::DEFAULT_PRICE_QUADRATIC_BASE.into(),
            },
        ),
        ContractError::Unauthorized {},
    );
}

/// `InvalidPriceScaling`: the grid must be positive and strictly increasing.
#[test]
fn a_decreasing_grid_is_reported() {
    let mut f = fixture();

    assert_error(
        set_grid(
            f.deps.as_mut(),
            &f.creator.clone(),
            PriceScaling {
                hour_1_price: 3_000_000u128.into(),
                hour_12_price: 2_000_000u128.into(),
                hour_24_price: 1_000_000u128.into(),
                quadratic_base: 500_000u128.into(),
            },
        ),
        ContractError::InvalidPriceScaling {},
    );
}

/// A zero price is invalid too: colouring is never free.
#[test]
fn a_zero_grid_is_reported() {
    let mut f = fixture();

    assert_error(
        set_grid(
            f.deps.as_mut(),
            &f.creator.clone(),
            PriceScaling {
                hour_1_price: 0u128.into(),
                hour_12_price: 2_000_000u128.into(),
                hour_24_price: 3_000_000u128.into(),
                quadratic_base: 4_000_000u128.into(),
            },
        ),
        ContractError::InvalidPriceScaling {},
    );
}

// ---- Payments ----
//
// The tile must exist for the payment rules to be reached: an unknown token id is
// reported as `TileNotFound` before any funds are examined, which one test below pins.

/// `InvalidPayment`: no funds at all. Colouring is never free.
#[test]
fn a_missing_payment_is_reported() {
    let mut f = fixture();
    minted(&mut f);

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "#FF0000", PIXEL_MIN_EXPIRATION)],
            &[],
        ),
        ContractError::InvalidPayment {
            expected: format!("exactly one {} coin", NATIVE_DENOM),
        },
    );
}

/// `InvalidPayment`: a denom other than the configured one.
#[test]
fn an_unknown_denom_is_reported() {
    let mut f = fixture();
    minted(&mut f);
    let funds = vec![Coin::new(1_000_000u128, "uosmo")];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "#FF0000", PIXEL_MIN_EXPIRATION)],
            &funds,
        ),
        ContractError::InvalidPayment {
            expected: format!("exactly one {} coin", NATIVE_DENOM),
        },
    );
}

/// `InvalidPayment`: two coins in the same message.
#[test]
fn a_multi_denom_payment_is_reported() {
    let mut f = fixture();
    minted(&mut f);
    let funds = vec![
        Coin::new(1_000_000u128, NATIVE_DENOM),
        Coin::new(1u128, "uosmo"),
    ];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "#FF0000", PIXEL_MIN_EXPIRATION)],
            &funds,
        ),
        ContractError::InvalidPayment {
            expected: format!("exactly one {} coin", NATIVE_DENOM),
        },
    );
}

/// `InvalidPayment`: the right denom, but the wrong amount.
#[test]
fn a_wrong_amount_is_reported() {
    let mut f = fixture();
    minted(&mut f);
    let below = tiles::defaults::constants::DEFAULT_PRICE_1_HOUR - 1;
    let funds = vec![Coin::new(below, NATIVE_DENOM)];

    assert_error(
        set_pixel(
            f.deps.as_mut(),
            &f.colourer.clone(),
            "1",
            vec![update(0, "#FF0000", PIXEL_MIN_EXPIRATION)],
            &funds,
        ),
        ContractError::InvalidPayment {
            expected: format!(
                "{} {}",
                tiles::defaults::constants::DEFAULT_PRICE_1_HOUR,
                NATIVE_DENOM
            ),
        },
    );
}
