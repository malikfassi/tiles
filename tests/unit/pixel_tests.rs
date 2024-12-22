use tiles::msg::PixelUpdate;
use crate::common::{mock_app, tiles_contract::TilesContract, vending_factory::VendingFactory};

#[test]
fn proper_initialization() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");
    assert!(!minter_addr.is_empty());
}

#[test]
fn test_set_pixel_color() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");

    // Create tiles contract
    let mut contract = TilesContract::new(&mut app, "owner");
    contract.contract_addr = minter_addr;

    // Mint a token
    contract.mint("1").unwrap();

    // Set pixel color
    let pixels = vec![PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration: 1000,
    }];
    contract.set_pixel_color("1", pixels).unwrap();
}

#[test]
fn test_set_pixel_color_unauthorized() {
    let (mut app, tiles_code_id, minter_code_id, factory_code_id) = mock_app();
    let mut factory = VendingFactory::new(&mut app, "owner");
    factory.instantiate(factory_code_id, tiles_code_id).unwrap();

    // Create minter
    let res = factory.create_minter(tiles_code_id).unwrap();
    let minter_addr = factory.parse_minter_response(&res).expect("Failed to parse minter address");

    // Create tiles contract
    let mut contract = TilesContract::new(&mut app, "owner");
    contract.contract_addr = minter_addr;

    // Try to set pixel color without minting
    let pixels = vec![PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration: 1000,
    }];
    contract.set_pixel_color("1", pixels).unwrap_err();
} 