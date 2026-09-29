use anyhow::Result;
use tiles::core::pricing::PriceScaling;

use crate::utils::{ContractAssertions, Launchpad};

/// Instantiation installs the CW721 collection, the default price grid and the
/// payment configuration, all of which must be readable straight afterwards.
#[test]
fn can_instantiate_contracts() -> Result<()> {
    let (launchpad, _) = Launchpad::setup()?;

    ContractAssertions::assert_price_scaling(
        &launchpad.app,
        &launchpad.tiles,
        &PriceScaling::default(),
    );

    let config = launchpad.tiles.query_config(&launchpad.app)?;
    assert!(
        !config.collection_payment_address.to_string().is_empty(),
        "the collection payment address must be configured"
    );
    assert!(
        !config.platform_payment_address.to_string().is_empty(),
        "the platform payment address must be configured"
    );

    Ok(())
}
