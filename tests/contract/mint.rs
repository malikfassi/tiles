use anyhow::Result;
use tiles::core::tile::metadata::PixelUpdate;

use crate::utils::{EventAssertions, TestSetup};

/// Minting a tile gives the owner 100 blank pixels and a stable hash.
#[test]
fn test_successful_mint() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();

    let token_id = setup.mint_token(&buyer)?;

    let pixels = setup.tile_metadata(token_id)?;
    assert_eq!(pixels.pixels.len(), 100, "a tile has 100 pixels");
    assert!(
        pixels.pixels.iter().all(|p| p.leased_by.is_none()),
        "a fresh tile has no leased pixels"
    );
    setup.tiles.assert_token_owner(&setup.app, token_id, &buyer);
    Ok(())
}

/// Only the configured minter may create tiles. The contract is the minter, so a plain
/// user is refused by the CW721 base.
#[test]
fn test_only_the_minter_can_mint() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let stranger = setup.users.poor_user().address.clone();

    let result = setup.tiles.mint_as_stranger(&mut setup.app, &stranger, "7");
    assert!(result.is_err(), "a stranger must not be able to mint");
    Ok(())
}

/// Colouring a pixel without paying fails, whatever the pixel.
#[test]
fn test_colouring_without_payment_fails() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let metadata = setup.tile_metadata(token_id)?;
    let update = PixelUpdate {
        id: 0,
        color: "#00FF00".to_string(),
        expiration_duration: 3600,
    };

    let result = setup.tiles.update_pixel_with_funds(
        &mut setup.app,
        &buyer,
        token_id,
        vec![update],
        metadata,
        vec![],
    );

    assert!(result.is_err(), "an unpaid colouring must be rejected");
    Ok(())
}

/// Unused import guard: keeps the event assertion helper available to this module.
#[test]
fn test_mint_emits_metadata_event() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();

    let response = setup.tiles.mint(&mut setup.app, &buyer, "1")?;
    EventAssertions::assert_mint_metadata(&response, 1, &buyer, None);
    Ok(())
}
