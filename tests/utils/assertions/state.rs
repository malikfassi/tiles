use cosmwasm_std::Addr;
use tiles::core::pricing::PriceScaling;

use crate::utils::{app::TestApp, contracts::tiles::TilesContract};

pub struct StateAssertions {}

impl StateAssertions {
    pub fn assert_balance(env: &TestApp, addr: &Addr, expected: u128) {
        let balance = env
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

    pub fn assert_price_scaling(actual: &PriceScaling, expected: &PriceScaling) {
        assert_eq!(
            actual.hour_1_price, expected.hour_1_price,
            "1 hour price mismatch"
        );
        assert_eq!(
            actual.hour_12_price, expected.hour_12_price,
            "12 hour price mismatch"
        );
        assert_eq!(
            actual.hour_24_price, expected.hour_24_price,
            "24 hour price mismatch"
        );
        assert_eq!(
            actual.quadratic_base, expected.quadratic_base,
            "Quadratic base mismatch"
        );
    }
}
