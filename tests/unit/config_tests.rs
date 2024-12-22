use cosmwasm_std::{Decimal, Uint128};
use crate::common::{mock_app, tiles_contract::TilesContract, vending_factory::VendingFactory};

#[test]
fn test_update_config() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");

    // Create tiles contract
    let mut contract = TilesContract::new(&mut app, "owner");
    contract.contract_addr = minter_addr;

    // Update config
    contract.update_config(
        Some("new_dev".to_string()),
        Some(Decimal::percent(10)),
        Some(Uint128::from(200_000_000u128)),
        None,
    ).unwrap();
}

#[test]
fn test_update_config_unauthorized() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");

    // Create tiles contract as non-owner
    let mut contract = TilesContract::new(&mut app, "non-owner");
    contract.contract_addr = minter_addr;

    // Try to update config as non-owner
    contract.update_config(
        Some("new_dev".to_string()),
        Some(Decimal::percent(10)),
        Some(Uint128::from(200_000_000u128)),
        None,
    ).unwrap_err();
} 