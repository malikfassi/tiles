use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

#[test]
fn can_set_pixel_color() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    test.ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update])?;

    Ok(())
}

#[test]
fn all_valid_updates_succeed() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 1,
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 2,
            color: "#0000FF".to_string(),
            expiration_duration: 3600,
        },
    ];

    test.ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates)?;

    Ok(())
}

#[test]
fn can_update_multiple_pixels() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 49,
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 99,
            color: "#0000FF".to_string(),
            expiration_duration: 3600,
        },
    ];

    test.ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates)?;

    Ok(())
} 