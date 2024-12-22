use cosmwasm_std::{coin, Coin, StdResult};

use tiles::defaults::constants::{
    MAX_PIXEL_UPDATES_PER_TILE, MAX_TILE_UPDATES_PER_MESSAGE,
};
use tiles::msg::{SetPixelColorMsg, TileUpdate, TileUpdates};

use crate::common::fixtures::TestSetup;

#[test]
fn test_set_pixel_color() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = TestSetup::new()
    else {
        panic!("Failed to setup test");
    };

    // Create update message
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: "1".to_string(),
            current_metadata: tiles.default_tile_metadata("1", &sender, app.block_info().time.seconds()),
            updates: TileUpdates {
                pixels: vec![tiles.default_pixel_update(0)],
            },
        }],
    };

    // Execute update
    let res = tiles.set_pixel_color(&mut app, &sender, msg);
    assert!(res.is_ok());
}

#[test]
fn test_set_pixel_color_multiple_tiles() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = TestSetup::new()
    else {
        panic!("Failed to setup test");
    };

    let timestamp = app.block_info().time.seconds();

    // Create update message
    let msg = SetPixelColorMsg {
        updates: vec![
            TileUpdate {
                tile_id: "1".to_string(),
                current_metadata: tiles.default_tile_metadata("1", &sender, timestamp),
                updates: TileUpdates {
                    pixels: vec![tiles.default_pixel_update(0)],
                },
            },
            TileUpdate {
                tile_id: "2".to_string(),
                current_metadata: tiles.default_tile_metadata("2", &sender, timestamp),
                updates: TileUpdates {
                    pixels: vec![tiles.default_pixel_update(0)],
                },
            },
        ],
    };

    // Execute update
    let res = tiles.set_pixel_color(&mut app, &sender, msg);
    assert!(res.is_ok());
}

#[test]
fn test_set_pixel_color_multiple_pixels() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = TestSetup::new()
    else {
        panic!("Failed to setup test");
    };

    // Create update message
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: "1".to_string(),
            current_metadata: tiles.default_tile_metadata("1", &sender, app.block_info().time.seconds()),
            updates: TileUpdates {
                pixels: vec![tiles.default_pixel_update(0), tiles.default_pixel_update(1)],
            },
        }],
    };

    // Execute update
    let res = tiles.set_pixel_color(&mut app, &sender, msg);
    assert!(res.is_ok());
}

#[test]
fn test_set_pixel_color_too_many_pixels() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = TestSetup::new()
    else {
        panic!("Failed to setup test");
    };

    // Create update message with too many pixels
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: "1".to_string(),
            current_metadata: tiles.default_tile_metadata("1", &sender, app.block_info().time.seconds()),
            updates: TileUpdates {
                pixels: (0..MAX_PIXEL_UPDATES_PER_TILE + 1)
                    .map(|id| tiles.default_pixel_update(id as u32))
                    .collect(),
            },
        }],
    };

    // Execute update
    let res = tiles.set_pixel_color(&mut app, &sender, msg);
    assert!(res.is_err());
}

#[test]
fn test_set_pixel_color_too_many_tiles() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = TestSetup::new()
    else {
        panic!("Failed to setup test");
    };

    let timestamp = app.block_info().time.seconds();

    // Create update message with too many tiles
    let msg = SetPixelColorMsg {
        updates: (0..MAX_TILE_UPDATES_PER_MESSAGE + 1)
            .map(|id| TileUpdate {
                tile_id: id.to_string(),
                current_metadata: tiles.default_tile_metadata(&id.to_string(), &sender, timestamp),
                updates: TileUpdates {
                    pixels: vec![tiles.default_pixel_update(0)],
                },
            })
            .collect(),
    };

    // Execute update
    let res = tiles.set_pixel_color(&mut app, &sender, msg);
    assert!(res.is_err());
}
