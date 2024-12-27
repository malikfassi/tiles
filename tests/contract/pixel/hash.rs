use crate::common::TestOrchestrator;
use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

#[test]
fn hash_is_updated_after_pixel_update() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let update = PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    test.ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update])?;

    let updated_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_ne!(initial_hash, updated_hash);

    Ok(())
}

#[test]
fn hash_is_updated_after_multiple_pixel_updates() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

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

    let updated_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_ne!(initial_hash, updated_hash);

    Ok(())
}

#[test]
fn hash_remains_unchanged_when_update_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let update = PixelUpdate {
        id: 100, // Invalid ID
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, vec![update]);

    test.assert_error_invalid_config(result, "Invalid pixel id: 100");

    let unchanged_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_eq!(initial_hash, unchanged_hash);

    Ok(())
}

#[test]
fn hash_remains_unchanged_when_batch_update_fails() -> Result<()> {
    let mut test = TestOrchestrator::new();
    let (owner, token_id) = test.setup_single_token()?;

    let initial_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;

    let updates = vec![
        PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 100, // Invalid ID
            color: "#00FF00".to_string(),
            expiration_duration: 3600,
        },
        PixelUpdate {
            id: 2,
            color: "#0000FF".to_string(),
            expiration_duration: 3600,
        },
    ];

    let result = test
        .ctx
        .tiles
        .update_pixel(&mut test.ctx.app, &owner, token_id, updates);

    test.assert_error_invalid_config(result, "Invalid pixel id: 100");

    let unchanged_hash = test.ctx.tiles.query_token_hash(&test.ctx.app, token_id)?;
    assert_eq!(initial_hash, unchanged_hash);

    Ok(())
} 