use anyhow::Result;
use cosmwasm_std::to_json_binary;

use crate::utils::TestSetup;

/// Standard CW721 behaviour, delegated to the base contract.
#[test]
fn test_transfer_nft() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let recipient = setup.users.pixel_operator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    setup
        .tiles
        .execute_transfer_nft(&mut setup.app, &buyer, &recipient, token_id.to_string())?;

    setup
        .tiles
        .assert_token_owner(&setup.app, token_id, &recipient);
    Ok(())
}

#[test]
fn test_send_nft_to_a_non_contract_fails() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let recipient = setup.users.pixel_operator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    let msg = to_json_binary("test_msg").expect("valid json");

    let result = setup.tiles.execute_send_nft(
        &mut setup.app,
        &buyer,
        &recipient,
        token_id.to_string(),
        msg,
    );

    assert!(result.is_err(), "sending to a plain address must fail");
    setup.tiles.assert_token_owner(&setup.app, token_id, &buyer);
    Ok(())
}

#[test]
fn test_approve_allows_operator_to_transfer() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let operator = setup.users.pixel_operator().address.clone();
    let recipient = setup.users.tile_contract_creator().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    setup.tiles.execute_approve(
        &mut setup.app,
        &buyer,
        &operator,
        token_id.to_string(),
        None,
    )?;

    setup.tiles.execute_transfer_nft(
        &mut setup.app,
        &operator,
        &recipient,
        token_id.to_string(),
    )?;

    setup
        .tiles
        .assert_token_owner(&setup.app, token_id, &recipient);
    Ok(())
}

#[test]
fn test_burn_removes_the_token() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().address.clone();
    let token_id = setup.mint_token(&buyer)?;

    setup
        .tiles
        .execute_burn(&mut setup.app, &buyer, token_id.to_string())?;

    let result = setup.tiles.query_owner(&setup.app, token_id);
    assert!(result.is_err(), "a burnt token has no owner");
    Ok(())
}

/// The minter is the contract itself, so a plain user calling `Mint` is refused by the
/// CW721 base. This goes through the base directly rather than the test helper, which
/// deliberately sends the call from the contract.
#[test]
fn test_only_minter_can_mint() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let stranger = setup.users.get_tile_creator().address.clone();

    let result = setup
        .tiles
        .mint_as_stranger(&mut setup.app, &stranger, "42");
    assert!(result.is_err(), "a non-minter must not be able to mint");
    Ok(())
}
