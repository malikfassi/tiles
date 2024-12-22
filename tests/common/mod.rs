use cosmwasm_std::Event;
use cw_multi_test::AppResponse;

pub mod fixtures;
pub mod mock;
pub mod tiles_contract;
pub mod vending_factory;

pub use fixtures::TestSetup;
pub use mock::init_modules;
pub use tiles_contract::TilesContract;
pub use vending_factory::VendingFactory;

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
