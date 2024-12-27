use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

#[test]
fn invalid_pixel_id_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 100, // Invalid ID (out of range)
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test.update_pixels(token_id, vec![update], &owner);
    test.assert_error_invalid_config(result, "Invalid pixel id: 100");
    Ok(())
}

#[test]
fn invalid_color_format_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "invalid".to_string(),
        expiration_duration: 3600,
    };

    let result = test.update_pixels(token_id, vec![update], &owner);
    test.assert_error_invalid_config(result, "Invalid color format: invalid");
    Ok(())
}

#[test]
fn invalid_expiration_duration_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    // Test too short
    let update_too_short = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 100,
    };
    let result = test.update_pixels(token_id, vec![update_too_short], &owner);
    test.assert_error_invalid_config(result, "Expiration duration 100 is less than minimum 3600");

    // Test too long
    let update_too_long = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 86401,
    };
    let result = test.update_pixels(token_id, vec![update_too_long], &owner);
    test.assert_error_invalid_config(
        result,
        "Expiration duration 86401 is greater than maximum 86400",
    );
    Ok(())
}

#[test]
fn duplicate_pixel_id_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

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

    let result = test.update_pixels(token_id, updates, &owner);
    test.assert_error_invalid_config(result, "Duplicate pixel id: 0");
    Ok(())
}

#[test]
fn batch_validation_fails_fast() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

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

    let result = test.update_pixels(token_id, updates, &owner);
    // Should fail on the first error encountered
    test.assert_error_invalid_config(result, "Invalid pixel id: 100");
    Ok(())
}
