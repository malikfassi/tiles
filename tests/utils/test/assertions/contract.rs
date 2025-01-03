use cosmwasm_std::Addr;
use tiles::core::pricing::PriceScaling;

use crate::utils::{contracts::tiles::TilesContract, core::app::TestApp};

pub struct ContractAssertions {}

impl ContractAssertions {
    pub fn assert_balance(app: &TestApp, addr: &Addr, expected: u128) {
        let balance = app
            .get_balance(addr, "ustars")
            .expect("Failed to get balance");
        assert_eq!(balance, expected, "Balance mismatch");
    }

    pub fn assert_token_owner(
        app: &TestApp,
        contract: &TilesContract,
        token_id: u32,
        expected_owner: &Addr,
    ) {
        let owner = contract
            .query_owner_of(app, token_id.to_string())
            .expect("Failed to query token owner");
        assert_eq!(
            owner.owner,
            expected_owner.to_string(),
            "Token owner mismatch"
        );
    }

    pub fn assert_token_hash(
        app: &TestApp,
        contract: &TilesContract,
        token_id: u32,
        expected_hash: &str,
    ) {
        let hash = contract
            .query_token_hash(app, token_id)
            .expect("Failed to query token hash");
        assert_eq!(hash, expected_hash, "Token hash mismatch");
    }

    pub fn assert_price_scaling(app: &TestApp, contract: &TilesContract, expected: &PriceScaling) {
        let actual = contract
            .query_price_scaling(app)
            .expect("Failed to query price scaling");
        assert_eq!(actual, *expected, "Price scaling mismatch");
    }
}
