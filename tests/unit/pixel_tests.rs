use crate::common::framework::TestContext;
use crate::common::users::UserRole;

#[test]
fn test_update_pixel() {
    let mut ctx = TestContext::new();

    // Mint a token as regular buyer
    let token_id = ctx.mint_as(UserRole::Buyer);

    // Update pixel color
    ctx.update_pixel_as(UserRole::Buyer, token_id, "#FF0000");

    // Assert ownership
    ctx.assert_token_owner(token_id, UserRole::Buyer);
}
