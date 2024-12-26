use crate::common::helpers::setup::TestSetup;
use cosmwasm_std::Addr;
use cw_multi_test::Executor;

#[test]
fn test_update_pixel() {
    let mut setup = TestSetup::new();
    let owner = Addr::unchecked("creator");

    // Mint a token through the minter
    setup
        .tiles
        .mint_through_minter(&mut setup.app, &owner, &setup.minter)
        .unwrap();

    // Update the pixel
    setup
        .tiles
        .update_pixel(&mut setup.app, &owner, 1u32, "#FF0000".to_string())
        .unwrap();
}
