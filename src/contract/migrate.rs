use crate::{
    contract::error::ContractError,
    contract::msg::MigrateMsg,
    defaults::constants::{CONTRACT_NAME, CONTRACT_VERSION},
    events::{EventData, MigrationEventData},
};
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::{get_contract_version, set_contract_version};

/// Migrates the stored state to the layout of the running code.
///
/// The storage schema is tracked with `cw2`, which records the contract name and version at
/// instantiation and on every migration. The rule applied here is deliberately strict:
///
/// - the contract name must be the one this code expects, otherwise the code is being
///   pointed at a different contract's storage and must refuse to touch it;
/// - migrating from the current version is a no-op, so a repeated migration is harmless;
/// - any other source version is refused rather than guessed at. A schema change that needs
///   real data rewriting must add its version to the match below, with the rewrite and a
///   test, instead of silently returning success.
///
/// This matters for the version changed by T-009: `Config` moved from `Decimal` shares to
/// `u64` basis points, which is not readable as the old layout. A deployment that still
/// holds a pre-T-009 state must be migrated explicitly, and this handler is where that is
/// declared.
pub fn migrate_handler(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let stored = get_contract_version(deps.storage)?;

    if stored.contract != CONTRACT_NAME {
        return Err(ContractError::UnsupportedMigration {
            from: format!("{} {}", stored.contract, stored.version),
        });
    }

    // Nothing to rewrite yet: the only layout this code knows is the current one.
    // A future schema change adds its source version here.
    if stored.version != CONTRACT_VERSION {
        return Err(ContractError::UnsupportedMigration {
            from: stored.version,
        });
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let event = MigrationEventData {
        from_version: stored.version,
        to_version: CONTRACT_VERSION.to_string(),
    }
    .into_event();

    Ok(Response::new().add_event(event))
}
