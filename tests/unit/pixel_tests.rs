use crate::common::helpers::setup::TestSetup;
use cosmwasm_std::Addr;

#[test]
fn test_update_pixel() {
    let mut setup = TestSetup::new();
    let owner = Addr::unchecked("creator");

    // Mint a token through the minter and get the response
    let mint_response = setup
        .tiles
        .mint_through_minter(&mut setup.app, &owner, &setup.minter)
        .unwrap();

    // Get the token ID from the response
    let token_id = 85u32; // For now hardcode to 85, later we can extract from response

    // Update the pixel using the minted token ID
    setup
        .tiles
        .update_pixel(&mut setup.app, &owner, token_id, "#FF0000".to_string())
        .unwrap();
}
