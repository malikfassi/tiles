use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, Fraction, MessageInfo, Response, Uint128};
use cw721::OwnerOfResponse;
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sg721_base::{msg::QueryMsg as Sg721QueryMsg, Sg721Contract};
use sg_std::StargazeMsgWrapper;
use std::collections::HashSet;

use crate::{
    contract::{error::ContractError, msg::QueryMsg, state::PRICE_SCALING},
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

    // Get token owner
    let owner: OwnerOfResponse = deps.querier.query_wasm_smart(
        env.contract.address.clone(),
        &QueryMsg::Base(Sg721QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        }),
    )?;

    // Get royalty info from collection info
    let collection_info = contract.collection_info.load(deps.storage)?;
    let royalty_info = collection_info
        .royalty_info
        .ok_or_else(|| ContractError::InvalidConfig("No royalty info configured".to_string()))?;

    let price_scaling = PRICE_SCALING.load(deps.storage)?;
    let current_time = env.block.time.seconds();
    let mut seen_ids = HashSet::new();
    let mut total_price = Uint128::zero();

    // Single pass: validate duplicates, validate updates, calculate price
    for update in &updates {
        // Check for duplicates
        if !seen_ids.insert(update.id) {
            return Err(ContractError::InvalidConfig(format!(
                "Duplicate pixel id: {}",
                update.id
            )));
        }

        // First validate the update integrity
        update.validate_integrity()?;

        // Then validate if it can be applied to the tile
        update.validate_for_tile(&current_metadata.pixels[update.id as usize], current_time)?;

        // Add to total price
        total_price += price_scaling.calculate_price(update.expiration_duration);
    }

    // Verify sent funds match total price
    let sent_funds = info.funds.iter().find(|c| c.denom == "ustars");
    match sent_funds {
        Some(coin) if coin.amount == total_price => (),
        Some(coin) if coin.amount > total_price => return Err(ContractError::InvalidFunds {}),
        Some(_) | None => return Err(ContractError::InsufficientFunds {}),
    }

    // Calculate payment distribution
    let royalty_amount = Uint128::from((total_price * royalty_info.share).u128());
    let owner_amount = total_price - royalty_amount;

    println!("Royalty info: {:?}", royalty_info);
    println!("Total price: {}", total_price);
    println!("Royalty amount: {}", royalty_amount);
    println!("Owner amount: {}", owner_amount);

    // Apply all updates at once
    current_metadata.apply_updates(updates, &info.sender, current_time);

    // Update token extension with new metadata hash
    token.extension.tile_hash = current_metadata.hash();
    contract.tokens.save(deps.storage, &token_id, &token)?;

    // Create bank messages for payment distribution
    let mut bank_msgs = vec![];

    // Send royalties to creator
    if !royalty_amount.is_zero() {
        let royalty_msg = BankMsg::Send {
            to_address: royalty_info.payment_address.to_string(),
            amount: vec![Coin {
                denom: "ustars".to_string(),
                amount: royalty_amount,
            }],
        };
        println!("Royalty message: {:?}", royalty_msg);
        bank_msgs.push(royalty_msg);
    }

    // Send remaining amount to token owner
    if !owner_amount.is_zero() {
        let owner_msg = BankMsg::Send {
            to_address: owner.owner,
            amount: vec![Coin {
                denom: "ustars".to_string(),
                amount: owner_amount,
            }],
        };
        println!("Owner message: {:?}", owner_msg);
        bank_msgs.push(owner_msg);
    }

    let response = Response::new()
        .add_messages(bank_msgs)
        .add_attribute("action", "set_pixel_color")
        .add_attribute("token_id", token_id)
        .add_attribute("sender", info.sender)
        .add_attribute("royalty_amount", format!("{}ustars", royalty_amount))
        .add_attribute("owner_amount", format!("{}ustars", owner_amount));

    println!("Response: {:?}", response);
    Ok(response)
}
