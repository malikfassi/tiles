use cosmwasm_std::{Event, Addr, Coin};
use cw_multi_test::ContractWrapper;
use sg_multi_test::StargazeApp;

pub mod mock;
pub mod tiles_contract;
pub mod vending_factory;

pub use mock::*;
pub use vending_factory::VendingFactory;

pub const NATIVE_DENOM: &str = "ustars";
pub const INITIAL_BALANCE: u128 = 1_000_000_000;

pub struct TestResponse {
    pub events: Vec<Event>,
}

impl TestResponse {
    pub fn new(response: cw_multi_test::AppResponse) -> Self {
        Self {
            events: response.events,
        }
    }
}
  