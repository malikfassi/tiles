//! Migration tests (T-011): the storage schema is versioned and migrations are strict.
//!
//! The contract refuses to migrate from any version it does not know rather than guessing
//! at a layout, and it refuses to touch storage that belongs to another contract. These
//! tests pin both rules, plus the fact that migrating from the current version is a no-op
//! and that the tile state survives.

use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier};
use cosmwasm_std::{Addr, Decimal, MemoryStorage, OwnedDeps};
use cw2::{get_contract_version, set_contract_version};
use cw721::msg::{CollectionExtensionMsg, RoyaltyInfoResponse};
use tiles::contract::contract::migrate;
use tiles::contract::error::ContractError;
use tiles::contract::execute::execute_handler;
use tiles::contract::instantiate::{instantiate_handler, InstantiateMsg};
use tiles::contract::msg::{ExecuteMsg, MigrateMsg, TileExecuteMsg};
use tiles::contract::state::{CONFIG, PRICE_SCALING};
use tiles::core::tile::metadata::TileMetadata;
use tiles::defaults::constants::{CONTRACT_NAME, CONTRACT_VERSION};

type Deps = OwnedDeps<MemoryStorage, MockApi, MockQuerier>;

fn account(label: &str) -> Addr {
    MockApi::default().addr_make(label)
}

/// Instantiates the contract and mints tile "1", so there is real state to preserve.
fn deployed() -> Deps {
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

    instantiate_handler(
        deps.as_mut(),
        mock_env(),
        message_info(&creator, &[]),
        InstantiateMsg {
            name: "Tiles".to_string(),
            symbol: "TILE".to_string(),
            collection_info_extension: Some(collection_info),
            minter: None,
            creator: Some(creator.to_string()),
            withdraw_address: None,
        },
    )
    .expect("instantiation succeeds");

    execute_handler(
        deps.as_mut(),
        mock_env(),
        message_info(&creator, &[]),
        ExecuteMsg::Mint {
            token_id: "1".to_string(),
            owner: creator.to_string(),
            token_uri: None,
            extension: tiles::core::Tile {
                tile_hash: TileMetadata::default().hash(),
                metadata: TileMetadata::default(),
            },
        },
    )
    .expect("minting succeeds");

    deps
}

/// Instantiation records the schema version, which is what a migration reads.
#[test]
fn instantiation_records_the_schema_version() {
    let deps = deployed();
    let stored = get_contract_version(deps.as_ref().storage).expect("the version is stored");

    assert_eq!(stored.contract, CONTRACT_NAME);
    assert_eq!(stored.version, CONTRACT_VERSION);
}

/// Migrating from the running version succeeds and reports the transition.
#[test]
fn migrating_from_the_current_version_is_a_no_op() {
    let mut deps = deployed();

    let response = migrate(deps.as_mut(), mock_env(), MigrateMsg {})
        .expect("migrating from the current version is allowed");

    let event = response
        .events
        .iter()
        .find(|e| e.ty == "migration")
        .expect("the migration event is emitted");
    let from = event
        .attributes
        .iter()
        .find(|a| a.key == "from_version")
        .expect("from_version is reported");
    let to = event
        .attributes
        .iter()
        .find(|a| a.key == "to_version")
        .expect("to_version is reported");

    assert_eq!(from.value, CONTRACT_VERSION);
    assert_eq!(to.value, CONTRACT_VERSION);
}

/// An unknown source version is refused, never guessed at.
#[test]
fn migrating_from_an_unknown_version_is_refused() {
    let mut deps = deployed();
    set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.0.1")
        .expect("recording an older version succeeds");

    let result = migrate(deps.as_mut(), mock_env(), MigrateMsg {});

    assert_eq!(
        result.unwrap_err(),
        ContractError::UnsupportedMigration {
            from: "0.0.1".to_string(),
        }
    );
}

/// Storage that belongs to a different contract is not touched.
#[test]
fn migrating_a_foreign_contract_is_refused() {
    let mut deps = deployed();
    set_contract_version(deps.as_mut().storage, "someone-else", CONTRACT_VERSION)
        .expect("recording a foreign name succeeds");

    let result = migrate(deps.as_mut(), mock_env(), MigrateMsg {});

    assert_eq!(
        result.unwrap_err(),
        ContractError::UnsupportedMigration {
            from: format!("someone-else {}", CONTRACT_VERSION),
        }
    );
}

/// A refused migration leaves the stored version untouched.
#[test]
fn a_refused_migration_does_not_change_the_stored_version() {
    let mut deps = deployed();
    set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.0.1")
        .expect("recording an older version succeeds");

    let _ = migrate(deps.as_mut(), mock_env(), MigrateMsg {});

    let stored = get_contract_version(deps.as_ref().storage).expect("the version is still stored");
    assert_eq!(
        stored.version, "0.0.1",
        "a refused migration must not rewrite the version"
    );
}

/// The tile state survives a migration: the storage keys are not renamed by it.
#[test]
fn the_state_survives_a_migration() {
    let mut deps = deployed();

    let config_before = CONFIG
        .load(deps.as_ref().storage)
        .expect("the config is stored");
    let scaling_before = PRICE_SCALING
        .load(deps.as_ref().storage)
        .expect("the price grid is stored");

    migrate(deps.as_mut(), mock_env(), MigrateMsg {}).expect("migrating succeeds");

    let config_after = CONFIG
        .load(deps.as_ref().storage)
        .expect("the config survived");
    let scaling_after = PRICE_SCALING
        .load(deps.as_ref().storage)
        .expect("the price grid survived");

    assert_eq!(
        config_after.collection_share_bps,
        config_before.collection_share_bps
    );
    assert_eq!(
        config_after.platform_share_bps,
        config_before.platform_share_bps
    );
    assert_eq!(scaling_after, scaling_before);
}

/// The collection is still usable after a migration: a pixel can still be coloured.
#[test]
fn colouring_still_works_after_a_migration() {
    let mut deps = deployed();
    let colourer = account("colourer");

    migrate(deps.as_mut(), mock_env(), MigrateMsg {}).expect("migrating succeeds");

    let price = tiles::defaults::constants::DEFAULT_PRICE_1_HOUR;
    let funds = vec![cosmwasm_std::Coin::new(
        price,
        tiles::defaults::constants::NATIVE_DENOM,
    )];

    execute_handler(
        deps.as_mut(),
        mock_env(),
        message_info(&colourer, &funds),
        ExecuteMsg::UpdateExtension {
            msg: TileExecuteMsg::SetPixelColor {
                token_id: "1".to_string(),
                current_metadata: TileMetadata::default(),
                updates: vec![tiles::core::tile::metadata::PixelUpdate {
                    id: 0,
                    color: "#FF0000".to_string(),
                    expiration_duration: tiles::defaults::constants::PIXEL_MIN_EXPIRATION,
                }],
            },
        },
    )
    .expect("the contract still works after a migration");
}
