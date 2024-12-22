use cosmwasm_std::Coin;
use tiles::msg::{SetPixelColorMsg, TileUpdate, TileUpdates, PixelUpdate};
use tiles::state::{TileMetadata, PixelData, Extension};

use crate::common::fixtures::{setup_test, TestSetup};

#[test]
fn test_set_pixel_color() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Token ID 1 was minted in setup_test
    let tile_id = "1".to_string();
    let creation_time = app.block_info().time.seconds();
    
    // Create initial pixels state (matches what was set during mint)
    let pixels = vec![PixelData::new_at_mint(0, sender.clone(), creation_time)];
    let current_metadata = TileMetadata {
        tile_id: tile_id.clone(),
        pixels: pixels.clone(),
    };

    // Update pixel color
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: tile_id.clone(),
            current_metadata: current_metadata.clone(),
            updates: TileUpdates {
                pixels: vec![PixelUpdate {
                    id: 0,
                    color: "#FF0000".to_string(),
                    expiration: app.block_info().time.seconds() + 3600,
                }],
            },
        }],
        max_message_size: 1024,
    };

    let res = tiles.set_pixel_color(&mut app, &sender, msg, vec![Coin::new(100_000, "ustars")]);
    if let Err(e) = &res {
        println!("Error setting pixel color: {:?}", e);
    }
    assert!(res.is_ok());
}

#[test]
fn test_set_pixel_color_invalid_color() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Create test data
    let tile_id = "1".to_string();
    let pixels = vec![PixelData::new_at_mint(0, sender.clone(), app.block_info().time.seconds())];
    let current_metadata = TileMetadata {
        tile_id: tile_id.clone(),
        pixels: pixels.clone(),
    };

    // Create initial extension with hash
    let _extension = Extension {
        tile_hash: Extension::generate_hash(&tile_id, &pixels),
    };

    // Try to update with invalid color
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: tile_id.clone(),
            current_metadata: current_metadata.clone(),
            updates: TileUpdates {
                pixels: vec![PixelUpdate {
                    id: 0,
                    color: "invalid".to_string(),
                    expiration: app.block_info().time.seconds() + 3600,
                }],
            },
        }],
        max_message_size: 1024,
    };

    let res = tiles.set_pixel_color(&mut app, &sender, msg, vec![Coin::new(100_000, "ustars")]);
    assert!(res.is_err());
}

#[test]
fn test_set_pixel_color_invalid_expiration() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Create test data
    let tile_id = "1".to_string();
    let pixels = vec![PixelData::new_at_mint(0, sender.clone(), app.block_info().time.seconds())];
    let current_metadata = TileMetadata {
        tile_id: tile_id.clone(),
        pixels: pixels.clone(),
    };

    // Create initial extension with hash
    let _extension = Extension {
        tile_hash: Extension::generate_hash(&tile_id, &pixels),
    };

    // Try to update with invalid expiration
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: tile_id.clone(),
            current_metadata: current_metadata.clone(),
            updates: TileUpdates {
                pixels: vec![PixelUpdate {
                    id: 0,
                    color: "#FF0000".to_string(),
                    expiration: app.block_info().time.seconds() + 31_536_001, // > 1 year
                }],
            },
        }],
        max_message_size: 1024,
    };

    let res = tiles.set_pixel_color(&mut app, &sender, msg, vec![Coin::new(100_000, "ustars")]);
    assert!(res.is_err());
}

#[test]
fn test_set_pixel_color_insufficient_funds() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Create test data
    let tile_id = "1".to_string();
    let pixels = vec![PixelData::new_at_mint(0, sender.clone(), app.block_info().time.seconds())];
    let current_metadata = TileMetadata {
        tile_id: tile_id.clone(),
        pixels: pixels.clone(),
    };

    // Create initial extension with hash
    let _extension = Extension {
        tile_hash: Extension::generate_hash(&tile_id, &pixels),
    };

    // Try to update with insufficient funds
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: tile_id.clone(),
            current_metadata: current_metadata.clone(),
            updates: TileUpdates {
                pixels: vec![PixelUpdate {
                    id: 0,
                    color: "#FF0000".to_string(),
                    expiration: app.block_info().time.seconds() + 3600,
                }],
            },
        }],
        max_message_size: 1024,
    };

    let res = tiles.set_pixel_color(&mut app, &sender, msg, vec![Coin::new(1, "ustars")]);
    assert!(res.is_err());
}

#[test]
fn test_set_pixel_color_message_too_large() {
    let Ok(TestSetup {
        mut app,
        sender,
        factory: _,
        tiles,
    }) = setup_test() else {
        panic!("Failed to setup test");
    };

    // Create test data
    let tile_id = "1".to_string();
    let pixels = vec![PixelData::new_at_mint(0, sender.clone(), app.block_info().time.seconds())];
    let current_metadata = TileMetadata {
        tile_id: tile_id.clone(),
        pixels: pixels.clone(),
    };

    // Create initial extension with hash
    let _extension = Extension {
        tile_hash: Extension::generate_hash(&tile_id, &pixels),
    };

    // Try to update with message size too large
    let msg = SetPixelColorMsg {
        updates: vec![TileUpdate {
            tile_id: tile_id.clone(),
            current_metadata: current_metadata.clone(),
            updates: TileUpdates {
                pixels: vec![PixelUpdate {
                    id: 0,
                    color: "#FF0000".to_string(),
                    expiration: app.block_info().time.seconds() + 3600,
                }],
            },
        }],
        max_message_size: 128 * 1024 + 1, // > 128KB
    };

    let res = tiles.set_pixel_color(&mut app, &sender, msg, vec![Coin::new(100_000, "ustars")]);
    assert!(res.is_err());
} 