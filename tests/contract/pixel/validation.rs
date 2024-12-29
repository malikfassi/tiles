use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

use crate::utils::TestSetup;

#[test]
fn invalid_pixel_id_fails() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    let update = PixelUpdate {
        id: 100, // Invalid ID (too high)
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = setup.update_pixel(&buyer.address, token_id, vec![update]);
    assert!(result.is_err(), "Expected error for invalid pixel id");
    Ok(())
}

#[test]
fn invalid_color_format_fails() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    let update = PixelUpdate {
        id: 0,
        color: "invalid".to_string(), // Invalid color format
        expiration_duration: 3600,
    };

    let result = setup.update_pixel(&buyer.address, token_id, vec![update]);
    assert!(result.is_err(), "Expected error for invalid color format");
    Ok(())
}

#[test]
fn invalid_expiration_duration_fails() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    // Test too short
    let update_too_short = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 100,
    };
    let result = setup.update_pixel(&buyer.address, token_id, vec![update_too_short]);
    assert!(result.is_err(), "Expected error for duration too short");

    // Test too long
    let update_too_long = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 86401,
    };
    let result = setup.update_pixel(&buyer.address, token_id, vec![update_too_long]);
    assert!(result.is_err(), "Expected error for duration too long");
    Ok(())
}

#[test]
fn duplicate_pixel_id_fails() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 0, // Duplicate ID
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result = setup.update_pixel(&buyer.address, token_id, updates);
    assert!(result.is_err(), "Expected error for duplicate pixel id");
    Ok(())
}

#[test]
fn batch_validation_fails_fast() -> Result<()> {
    let (mut setup, token_id) = TestSetup::with_minted_token()?;
    let buyer = setup.users.get_buyer().clone();

    // Test batch with multiple invalid updates
    let updates = vec![
        PixelUpdate {
            id: 100, // Invalid ID
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 0,
            color: "invalid".to_string(), // Invalid color
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration_duration: 100, // Invalid duration
        },
    ];

    let result = setup.update_pixel(&buyer.address, token_id, updates);
    assert!(result.is_err(), "Expected error for invalid pixel id");
    Ok(())
}
