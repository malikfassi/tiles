use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use cw721::OwnerOfResponse;
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;
use std::collections::HashSet;

use crate::{
    contract::{error::ContractError, msg::QueryMsg, state::PRICE_SCALING},
    core::tile::{
        metadata::{PixelUpdate, TileMetadata},
        Tile,
    },
    events::{
        EventData, MetadataUpdateEventData, PaymentDistributionEventData, PixelUpdateEventData,
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
        return Err(ContractError::MetadataHashMismatch {});
    }

    // Get token owner
    let owner_query = QueryMsg::OwnerOf {
        token_id: token_id.clone(),
        include_expired: None,
    };
    let owner: OwnerOfResponse = deps
        .querier
        .query_wasm_smart(env.contract.address.clone(), &owner_query)?;

    let price_scaling = PRICE_SCALING.load(deps.storage)?;
    let current_time = env.block.time.seconds();
    let mut seen_ids = HashSet::new();
    let mut total_price = Uint128::zero();

    // Single pass: validate duplicates, validate updates, calculate price
    for update in &updates {
        // Check for duplicates
        if !seen_ids.insert(update.id) {
            return Err(ContractError::DuplicatePixelId { id: update.id });
        }

        // First validate the update integrity
        update.validate_integrity()?;

        // Then validate if it can be applied to the tile
        update.validate_for_tile(&current_metadata.pixels[update.id as usize], current_time)?;

        // Add to total price
        total_price += price_scaling.calculate_price(update.expiration_duration);
    }

    // Verify sent funds match total price
    if info.funds.is_empty() || info.funds[0].amount != total_price {
        return Err(ContractError::InsufficientFunds {});
    }

    // Get royalty info from collection info
    let collection_info = contract.collection_info.load(deps.storage)?;
    let royalty_info = collection_info
        .royalty_info
        .ok_or(ContractError::MissingRoyaltyInfo {})?;

    // Calculate payment distribution
    let royalty_amount = total_price * royalty_info.share;
    let owner_amount = total_price - royalty_amount;

    // Create bank messages for payment distribution
    let bank_msgs: Vec<CosmosMsg<StargazeMsgWrapper>> = vec![
        cosmwasm_std::BankMsg::Send {
            to_address: royalty_info.payment_address.to_string(),
            amount: vec![cosmwasm_std::Coin {
                denom: info.funds[0].denom.clone(),
                amount: royalty_amount,
            }],
        }
        .into(),
        cosmwasm_std::BankMsg::Send {
            to_address: owner.owner,
            amount: vec![cosmwasm_std::Coin {
                denom: info.funds[0].denom.clone(),
                amount: owner_amount,
            }],
        }
        .into(),
    ];

    // Create events for each pixel update
    let pixel_events = updates
        .iter()
        .map(|update| {
            let event = PixelUpdateEventData {
                token_id: token_id.clone(),
                pixel_id: update.id,
                color: update.color.clone(),
                expiration_duration: update.expiration_duration,
                expiration_timestamp: current_time + update.expiration_duration,
                last_updated_by: info.sender.clone(),
                last_updated_at: current_time,
            }
            .into_event();
            println!("Pixel update event: {:?}", event);
            event
        })
        .collect::<Vec<_>>();

    // Apply all updates at once
    current_metadata.apply_updates(updates, &info.sender, current_time);

    // Create metadata updated event
    let metadata_event = MetadataUpdateEventData {
        token_id: token_id.clone(),
        resulting_hash: current_metadata.hash(),
    }
    .into_event();
    println!("Metadata update event: {:?}", metadata_event);

    // Create payment distribution event
    let payment_event = PaymentDistributionEventData {
        token_id: token_id.clone(),
        sender: info.sender.clone(),
        royalty_amount: royalty_amount.u128(),
        owner_amount: owner_amount.u128(),
    }
    .into_event();
    println!("Payment distribution event: {:?}", payment_event);

    // Update token extension with new metadata hash
    token.extension.tile_hash = current_metadata.hash();
    contract.tokens.save(deps.storage, &token_id, &token)?;

    let response = Response::new()
        .add_messages(bank_msgs)
        .add_events(pixel_events)
        .add_event(metadata_event)
        .add_event(payment_event);

    Ok(response)
}
