use std::collections::HashSet;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, state::CONFIG},
    core::tile::{
        metadata::{PixelUpdate, TileMetadata},
        Tile,
    },
};

pub fn set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    mut current_metadata: TileMetadata,
    updates: Vec<PixelUpdate>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();

    // Verify current metadata hash matches stored hash
    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    if token.extension.tile_hash != current_metadata.hash() {
        return Err(ContractError::HashMismatch {});
    }

    let config = CONFIG.load(deps.storage)?;
    let current_time = env.block.time.seconds();
    let mut seen_ids = HashSet::new();
    let mut total_price = Uint128::zero();

    // Single pass: validate duplicates, calculate price, and apply updates
    for update in &updates {
        // Check for duplicates
        if !seen_ids.insert(update.id) {
            return Err(ContractError::InvalidConfig(format!(
                "Duplicate pixel id: {}",
                update.id
            )));
        }

        // Add to total price
        total_price += config.price_scaling.calculate_price(update.expiration_duration);
    }

    // Verify sent funds match total price
    let sent_funds = info.funds.iter().find(|c| c.denom == "ustars");
    match sent_funds {
        Some(coin) if coin.amount == total_price => (),
        _ => {
            return Err(ContractError::InvalidFunds(format!(
                "Expected {} ustars, got {:?}",
                total_price, info.funds
            )));
        }
    }

    // Apply all updates at once
    current_metadata.apply_updates(updates, &info.sender, current_time);

    // Update token extension with new metadata hash
    token.extension.tile_hash = current_metadata.hash();
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "set_pixel_color")
        .add_attribute("token_id", token_id)
        .add_attribute("sender", info.sender))
}
