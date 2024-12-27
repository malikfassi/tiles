use crate::common::{launchpad::Launchpad, TestApp};

#[test]
fn test_setup() {
    let ctx = Launchpad::new();
    assert!(!ctx.factory.contract_addr.as_str().is_empty());
    assert!(!ctx.minter.contract_addr.as_str().is_empty());
    assert!(!ctx.tiles.contract_addr.as_str().is_empty());
}

#[test]
fn test_setup_with_app() {
    let mut app = TestApp::new();
    let tiles_code_id = app.store_tiles_code().unwrap();
    let factory_code_id = app.store_vending_factory_code().unwrap();
    let minter_code_id = app.store_vending_minter_code().unwrap();
    assert!(tiles_code_id > 0);
    assert!(factory_code_id > 0);
    assert!(minter_code_id > 0);
}
