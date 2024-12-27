use crate::common::launchpad::Launchpad;
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
