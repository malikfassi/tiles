use anyhow::Result;
use tiles::core::pricing::PriceScaling;

use crate::utils::{ContractAssertions, EventAssertions, Launchpad, TestSetup};

#[test]
fn can_instantiate_contracts() -> Result<()> {
    // Setup contracts and get instantiation response
    let (launchpad, response) = Launchpad::setup()?;

    // Verify instantiation event was emitted
    EventAssertions::assert_instantiate_price_scaling(&response, &PriceScaling::default())?;

    // Query contract and verify price scaling
    ContractAssertions::assert_price_scaling(
        &launchpad.app,
        &launchpad.tiles,
        &PriceScaling::default(),
    );

    Ok(())
}
