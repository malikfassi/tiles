use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::{
    contract::{error::ContractError, state::TILE_CONFIG},
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
    current_metadata: TileMetadata,
    updates: Vec<PixelUpdate>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract: Sg721Contract<Tile> = Sg721Contract::default();

    // Verify current metadata hash matches stored hash
    let token = contract.tokens.load(deps.storage, &token_id)?;
    if token.extension.tile_hash != current_metadata.hash() {
        return Err(ContractError::HashMismatch {});
    }

    // Calculate total price for updates
    let config = TILE_CONFIG.load(deps.storage)?;
    let total_price = config.price_scaling.calculate_total_price(
        &updates.iter().map(|u| u.expiration).collect::<Vec<_>>(),
        env.block.time.seconds(),
    );

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

    // Apply updates to metadata
    let mut new_metadata = current_metadata;
    for update in updates {
        update.apply(&mut new_metadata, &info.sender, env.block.time.seconds());
    }

    // Update token extension with new metadata hash
    let mut token = token;
    token.extension.tile_hash = new_metadata.hash();
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "set_pixel_color")
        .add_attribute("token_id", token_id)
        .add_attribute("sender", info.sender))
}
