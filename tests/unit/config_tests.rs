use crate::common::helpers::setup::{TestSetup, UserType};

#[test]
fn test_update_minter() {
    let mut setup = TestSetup::new();

    // Mint a token as regular buyer
    setup.mint_as(UserType::Buyer);
}
