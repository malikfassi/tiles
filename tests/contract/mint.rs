use anyhow::Result;
use tiles::{core::tile::metadata::TileMetadata, defaults::constants::MINT_PRICE};

use crate::common::{EventAssertions, TestContext};

#[test]
fn test_successful_mint() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer().clone();
    let initial_balance = ctx
        .app
        .get_balance(&buyer.address, "ustars")
        .expect("Failed to get balance");

    let response = ctx.mint_token(&buyer.address)?;
    let token_id = EventAssertions::extract_token_id(&response);

    ctx.assert_balance(&buyer.address, "ustars", initial_balance - MINT_PRICE);
    ctx.tiles
        .assert_token_owner(&ctx.app, token_id, &buyer.address);
    EventAssertions::assert_mint_metadata(&response, token_id, &buyer.address, None);

    Ok(())
}

#[test]
fn test_insufficient_funds() -> Result<()> {
    let mut ctx = TestContext::new();
    let user = ctx.users.poor_user().clone();

    let result = ctx.mint_token(&user.address);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_default_hash() -> Result<()> {
    let mut ctx = TestContext::new();
    let buyer = ctx.users.get_buyer().clone();

    let response = ctx.mint_token(&buyer.address)?;
    let token_id = EventAssertions::extract_token_id(&response);

    // Query the token's hash
    let token_hash = ctx.tiles.query_token_hash(&ctx.app, token_id)?;

    // Compute the expected default hash
    let default_metadata = TileMetadata::default();
    let expected_hash = default_metadata.hash();

    assert_eq!(
        token_hash, expected_hash,
        "Newly minted token should have the default metadata hash"
    );

    EventAssertions::assert_mint_metadata(
        &response,
        token_id,
        &buyer.address,
        Some(&expected_hash),
    );

    Ok(())
}
