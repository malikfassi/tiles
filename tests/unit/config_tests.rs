use cosmwasm_std::Addr;

use crate::common::fixtures::setup_test;

#[test]
fn test_update_config() -> anyhow::Result<()> {
    let mut setup = setup_test()?;
    let admin = setup.sender;

    // Query initial config
    let config = setup.tiles.query_config(&setup.app)?;
    assert_eq!(config.admin, admin);

    // Update config
    setup.tiles.update_config(
        &mut setup.app,
        &admin,
        Some("new_dev".to_string()),
        Some(5),
        Some(1_000_000),
        Some(tiles::contract::state::PriceScaling {
            hour_1_price: 100_000_000,
            hour_12_price: 200_000_000,
            hour_24_price: 300_000_000,
            quadratic_base: 400_000_000,
        }),
    )?;

    // Query updated config
    let config = setup.tiles.query_config(&setup.app)?;
    assert_eq!(config.dev_address, Addr::unchecked("new_dev"));
    assert_eq!(config.dev_fee_percent, 5);
    assert_eq!(config.base_price, 1_000_000);

    Ok(())
}
