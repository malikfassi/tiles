use anyhow::Result;
use tiles::defaults::constants::MINT_PRICE;

use crate::utils::{events::EventParser, ResponseAssertions, StateAssertions, TestSetup};

#[test]
fn test_successful_mint() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();
    let initial_balance = setup
        .app
        .get_balance(&buyer.address, "ustars")
        .expect("Failed to get balance");

    let response = setup.minter.mint(&mut setup.app, &buyer.address)?;
    let token_id = EventParser::extract_token_id(&response)?;
    ResponseAssertions::assert_mint_metadata(&response, token_id, &buyer.address, None);

    StateAssertions::assert_balance(&setup.app, &buyer.address, initial_balance - MINT_PRICE);
    StateAssertions::assert_token_owner(&setup.app, &setup.tiles, token_id, &buyer.address);

    Ok(())
}

#[test]
fn test_insufficient_funds() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let user = setup.users.poor_user().clone();

    let result = setup.minter.mint(&mut setup.app, &user.address);
    assert!(result.is_err());

    Ok(())
}
