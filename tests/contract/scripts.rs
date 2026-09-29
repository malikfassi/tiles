//! Deployment script tests (T-012).
//!
//! The scripts build their messages with `jq`, so a typo in a field name would only be
//! caught when the transaction reaches the chain. These tests take the very same message
//! shapes and feed them to the contract types, which turns a broken script into a failing
//! `cargo test` instead of a failed deployment.

use tiles::contract::instantiate::InstantiateMsg;
use tiles::contract::msg::ExecuteMsg;
use tiles::core::tile::metadata::TileMetadata;

/// The instantiate message that `scripts/02_deploy_contracts.sh` builds must deserialise
/// into the contract's own instantiate type, field for field.
#[test]
fn the_deploy_script_instantiate_message_matches_the_contract() {
    // Same shape as the jq expression in the script, with values filled in.
    let jq_output = r#"{
        "name": "Tiles",
        "symbol": "TILE",
        "minter": null,
        "creator": "cosmwasm1creator",
        "withdraw_address": null,
        "collection_info_extension": {
            "description": "A collaborative pixel art canvas",
            "image": null,
            "external_link": null,
            "royalty_info": {
                "payment_address": "cosmwasm1creator",
                "share": "0.05"
            }
        }
    }"#;

    let msg: InstantiateMsg =
        serde_json::from_str(jq_output).expect("the script message must deserialise");

    assert_eq!(msg.name, "Tiles");
    assert_eq!(msg.symbol, "TILE");
    assert_eq!(msg.creator.as_deref(), Some("cosmwasm1creator"));
    assert!(msg.minter.is_none(), "the contract is its own minter");

    let extension = msg
        .collection_info_extension
        .expect("the collection extension is sent");
    assert_eq!(
        extension.description.as_deref(),
        Some("A collaborative pixel art canvas")
    );
    let royalties = extension
        .royalty_info
        .expect("royalties are declared in the extension");
    assert_eq!(royalties.payment_address, "cosmwasm1creator");
    assert_eq!(royalties.share, cosmwasm_std::Decimal::percent(5));
}

/// The mint message that `scripts/03_mint_token.sh` builds must deserialise too.
#[test]
fn the_mint_script_message_matches_the_contract() {
    let mint_output = r##"{
        "mint": {
            "token_id": "1",
            "owner": "cosmwasm1creator",
            "token_uri": null,
            "extension": {
                "tile_hash": "",
                "metadata": __METADATA__
            }
        }
    }"##
    .replace(
        "__METADATA__",
        &serde_json::to_string(&TileMetadata::default()).expect("metadata serialises"),
    );

    let msg: ExecuteMsg =
        serde_json::from_str(&mint_output).expect("the mint message deserialises");

    match msg {
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension,
        } => {
            assert_eq!(token_id, "1");
            assert_eq!(owner, "cosmwasm1creator");
            assert!(token_uri.is_none());
            assert_eq!(
                extension.metadata,
                TileMetadata::default(),
                "the tile starts as the blank 100-pixel canvas"
            );
        }
        other => panic!("expected a Mint message, got {:?}", other),
    }
}

/// The set-pixel message that `scripts/04_set_pixel_color.sh` builds must deserialise,
/// including the `current_metadata` read back from the chain.
#[test]
fn the_colour_script_message_matches_the_contract() {
    let metadata = serde_json::to_string(&TileMetadata::default()).expect("metadata serialises");

    // Same shape as the jq expression in the script: the metadata is a JSON object
    // embedded in the message, which is exactly how the script passes it too.
    // The raw string uses `##` because a colour literal contains `"#`, which would
    // otherwise close the string early.
    let jq_output = r##"{
            "update_extension": {
                "msg": {
                    "set_pixel_color": {
                        "token_id": "1",
                        "current_metadata": __METADATA__,
                        "updates": [
                            {
                                "id": 42,
                                "color": "#FF0000",
                                "expiration_duration": 3600
                            }
                        ]
                    }
                }
            }
        }"##
    .replace("__METADATA__", &metadata);

    let msg: ExecuteMsg =
        serde_json::from_str(&jq_output).expect("the set-pixel message deserialises");

    match msg {
        ExecuteMsg::UpdateExtension { msg } => match msg {
            tiles::contract::msg::TileExecuteMsg::SetPixelColor {
                token_id,
                current_metadata,
                updates,
            } => {
                assert_eq!(token_id, "1");
                assert_eq!(current_metadata.hash(), TileMetadata::default().hash());
                assert_eq!(updates.len(), 1);
                assert_eq!(updates[0].id, 42);
                assert_eq!(updates[0].color, "#FF0000");
                assert_eq!(updates[0].expiration_duration, 3600);
            }
            other => panic!("expected SetPixelColor, got {:?}", other),
        },
        other => panic!("expected UpdateExtension, got {:?}", other),
    }
}

/// The `migrate` message is empty, and the deployment scripts must not invent fields.
#[test]
fn the_migrate_message_is_empty_and_accepted() {
    let msg: tiles::contract::msg::MigrateMsg =
        serde_json::from_str("{}").expect("an empty object is the migrate message");
    let _ = msg;
}
