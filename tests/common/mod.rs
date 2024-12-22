use cosmwasm_std::Event;
use cw_multi_test::AppResponse;

pub mod fixtures;
pub mod mock;
pub mod tiles_contract;
pub mod vending_factory;

pub use mock::*;
pub use tiles_contract::TilesContract;
pub use vending_factory::VendingFactoryContract;

pub const NATIVE_DENOM: &str = "ustars";
pub const INITIAL_BALANCE: u128 = 1_000_000_000;

pub struct TestResponse {
    pub events: Vec<Event>,
}

impl TestResponse {
    pub fn new(response: AppResponse) -> Self {
        Self {
            events: response.events,
        }
    }
}
