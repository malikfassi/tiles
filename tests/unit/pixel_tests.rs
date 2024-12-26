use crate::common::helpers::setup::{TestSetup, UserType};

#[test]
fn test_update_pixel() {
    let mut setup = TestSetup::new();

    // Mint a token as regular buyer
    let token_id = setup.mint_as(UserType::Buyer);

    // Update pixel color using the minted token ID
    setup.update_pixel_as(UserType::Buyer, token_id, "#FF0000");
}
