use cosmwasm_std::Coin;
use cw721::TokensResponse;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;

use crate::common::{fixtures::setup_test, NATIVE_DENOM};

#[test]
fn test_set_pixel_color() -> anyhow::Result<()> {
    let mut setup = setup_test()?;

    // Query all tokens
    let tokens: TokensResponse = setup.app.wrap().query_wasm_smart(
        setup.tiles.contract_addr.clone(),
        &tiles::contract::msg::QueryMsg::Sg721(
            Sg721QueryMsg::Tokens {
                owner: setup.sender.to_string(),
                start_after: None,
                limit: None,
            }
        ),
    )?;

    assert_eq!(tokens.tokens.len(), 1);
    let token_id = tokens.tokens[0].clone();

    // Set pixel color
    setup.tiles.set_pixel_color(
        &mut setup.app,
        &setup.sender,
        token_id.clone(),
        "#FF0000".to_string(),
        0,
        60,
        vec![Coin::new(100_000_000, NATIVE_DENOM)],
    )?;

    // Query pixel state
    let state: tiles::contract::msg::PixelStateResponse = setup.app.wrap().query_wasm_smart(
        setup.tiles.contract_addr,
        &tiles::contract::msg::QueryMsg::Extension(
            tiles::contract::msg::Extension::PixelState {
                token_id,
                position: 0,
            }
        ),
    )?;

    assert_eq!(state.color, "#FF0000");
    assert_eq!(state.position, 0);
    assert!(state.expiration > 0);

    Ok(())
}
