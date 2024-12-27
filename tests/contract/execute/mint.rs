#[cfg(test)]
use crate::common::launchpad::Launchpad;
use tiles::core::tile::metadata::TileMetadata;
use tiles::defaults::constants::MINT_PRICE;

#[test]
fn test_mint_success() {
    let mut ctx = Launchpad::new();
    let buyer = ctx.users.get_buyer();
    let initial_balance = ctx.app.get_balance(&buyer.address, "ustars").unwrap();

    let token_id = ctx.minter.mint(&mut ctx.app, &buyer.address).unwrap();

    let final_balance = ctx.app.get_balance(&buyer.address, "ustars").unwrap();
    assert_eq!(initial_balance - MINT_PRICE, final_balance);

    ctx.tiles
        .assert_token_owner(&ctx.app, token_id, &buyer.address);
}

#[test]
fn test_mint_insufficient_funds() {
    let mut ctx = Launchpad::new();
    let user = ctx.users.poor_user();

    let result = ctx.minter.mint(&mut ctx.app, &user.address);
    assert!(result.is_err());
}

#[test]
fn test_mint_default_hash() {
    let mut ctx = Launchpad::new();
    let buyer = ctx.users.get_buyer();

    let token_id = ctx.minter.mint(&mut ctx.app, &buyer.address).unwrap();

    // Query the token's hash
    let token_hash = ctx.tiles.query_token_hash(&ctx.app, token_id).unwrap();

    // Compute the expected default hash
    let default_metadata = TileMetadata::default();
    let expected_hash = default_metadata.hash();

    assert_eq!(
        token_hash, expected_hash,
        "Newly minted token should have the default metadata hash"
    );
}
