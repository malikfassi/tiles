/// Test application wrapper around the standard `cw_multi_test::App`.
///
/// The contract targets the Cosmos Hub and speaks plain CosmWasm: no custom chain
/// message wrapper is needed any more, which is why this is `App` and not `StargazeApp`.
use anyhow::Result;
use cosmwasm_std::{Addr, Timestamp};
use cw_multi_test::{App, AppBuilder, Contract};

use tiles::defaults::constants::NATIVE_DENOM;

/// Denomination used in tests, in micro units.
pub const TEST_DENOM: &str = NATIVE_DENOM;

pub struct TestApp {
    app: App,
}

impl TestApp {
    /// Creates a new test application with default configuration.
    pub fn new() -> Self {
        Self {
            app: AppBuilder::new().build(|_, _, _| {}),
        }
    }

    pub fn get_balance(&self, address: &Addr, denom: &str) -> Result<u128> {
        Ok(self
            .app
            .wrap()
            .query_balance(address.to_string(), denom)?
            .amount
            .u128())
    }

    /// Advances the blockchain time by the specified number of seconds.
    pub fn advance_time(&mut self, seconds: u64) {
        self.app.update_block(|block| {
            block.time = block.time.plus_seconds(seconds);
            block.height += 1;
        });
    }

    pub fn store_code(&mut self, contract: Box<dyn Contract<cosmwasm_std::Empty>>) -> u64 {
        self.app.store_code(contract)
    }

    pub fn inner(&self) -> &App {
        &self.app
    }

    pub fn inner_mut(&mut self) -> &mut App {
        &mut self.app
    }

    /// Moves the block time to a fixed, predictable timestamp.
    pub fn set_time(&mut self, seconds: u64) {
        self.app.update_block(|block| {
            block.time = Timestamp::from_seconds(seconds);
            block.height += 1;
        });
    }
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}
