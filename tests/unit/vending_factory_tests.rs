use crate::common::{mock_app, vending_factory::VendingFactory};

#[test]
fn proper_instantiation_via_factory() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");
    assert!(!minter_addr.is_empty());
}

#[test]
fn test_minter_params() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");
    assert!(!minter_addr.is_empty());
}

#[test]
fn test_mint_limits() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");
    assert!(!minter_addr.is_empty());
} 