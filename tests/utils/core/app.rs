/// Test application wrapper that provides a simulated blockchain environment.
/// This is the lowest level of the test infrastructure, providing basic blockchain
/// operations and state management.
use anyhow::Result;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_multi_test::{Contract, Executor};
use sg_multi_test::StargazeApp;
use sg_std::{StargazeMsgWrapper, GENESIS_MINT_START_TIME, NATIVE_DENOM};

pub struct TestApp {
    app: StargazeApp,
}

impl TestApp {
    /// Creates a new test application with default configuration.
    pub fn new() -> Self {
        Self {
            app: StargazeApp::default(),
        }
    }

    /// Gets the balance of a specific address in the given denomination.
    ///
    /// # Arguments
    /// * `address` - The address to check the balance for
    /// * `denom` - The denomination of the tokens to check
    pub fn get_balance(&self, address: &Addr, denom: &str) -> Result<u128> {
        Ok(self
            .app
            .wrap()
            .query_balance(address.to_string(), denom)?
            .amount
            .u128())
    }

    /// Advances the blockchain time by the specified number of seconds.
    ///
    /// # Arguments
    /// * `seconds` - Number of seconds to advance the time by
    pub fn advance_time(&mut self, seconds: u64) {
        self.app.update_block(|block| {
            block.time = block.time.plus_seconds(seconds);
            block.height += 1;
        });
    }

    /// Stores contract code in the test environment
    ///
    /// # Arguments
    /// * `contract` - The contract code to store
    ///
    /// # Returns
    /// * `u64` - The code ID of the stored contract
    pub fn store_code(&mut self, contract: Box<dyn Contract<StargazeMsgWrapper>>) -> u64 {
        self.app.store_code(contract)
    }

    /// Provides access to the underlying App instance.
    pub fn inner(&self) -> &StargazeApp {
        &self.app
    }

    /// Provides mutable access to the underlying App instance.
    pub fn inner_mut(&mut self) -> &mut StargazeApp {
        &mut self.app
    }
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}
