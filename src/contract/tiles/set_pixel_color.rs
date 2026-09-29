use crate::{
    contract::{
        error::ContractError,
        instantiate::TilesContract,
        state::{CONFIG, PRICE_SCALING},
    },
    core::{
        tile::metadata::{PixelUpdate, TileMetadata},
        validation::{
            is_valid_hex_color, split_payment, validate_updates, validate_updates_for_tile,
        },
    },
    events::{
        EventData, MetadataUpdateEventData, PaymentDistributionEventData, PixelUpdateEventData,
    },
};
use cosmwasm_std::{Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

/// Pay to write pixels on a tile.
///
/// Anyone may call this: the sender does not need to own the tile (ADR 0004).
/// A pixel under an active lease can only be written by the address holding that lease.
pub fn set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    current_metadata: TileMetadata,
    updates: Vec<PixelUpdate>,
) -> Result<Response, ContractError> {
    // 1. Message validation.
    validate_updates(&updates)?;

    // 2. Load the tile and use the hash as an optimistic lock.
    let contract = TilesContract::default();
    let mut token = contract
        .config
        .nft_info
        .load(deps.storage, &token_id)
        .map_err(|_| ContractError::TileNotFound {
            token_id: token_id.clone(),
        })?;

    let mut metadata = token.extension.metadata.clone();
    if metadata.hash() != current_metadata.hash() {
        return Err(ContractError::MetadataHashMismatch {});
    }

    // 3. Business rules: is any target pixel still under someone else's lease?
    let now = env.block.time.seconds();
    validate_updates_for_tile(&token_id, &metadata, &updates, &info.sender, now)?;

    // 4. Price: duration grid, floored by the configured minimum.
    let config = CONFIG.load(deps.storage)?;
    let price_scaling = PRICE_SCALING.load(deps.storage)?;
    let mut total = Uint128::zero();
    for update in &updates {
        total += price_scaling.calculate_price(update.expiration_duration);
    }
    if total < config.minimum_price {
        total = config.minimum_price;
    }

    // 5. Payment: exactly one coin, of the expected amount, sent to the contract.
    let denom = crate::defaults::constants::NATIVE_DENOM;
    let paid = single_payment(&info, denom)?;
    if paid != total {
        return Err(ContractError::InvalidPayment {
            expected: format!("{} {}", total, denom),
        });
    }

    // 6. Split: collection, platform, then the owner receives the remainder,
    //    so the three amounts always add up to the exact total.
    let (collection_amount, platform_amount, owner_amount) = split_payment(
        total,
        config.collection_share_percent,
        config.platform_share_percent,
    );

    let mut messages: Vec<CosmosMsg> = Vec::with_capacity(3);
    if !collection_amount.is_zero() {
        messages.push(bank_send(
            config.collection_payment_address.to_string(),
            denom,
            collection_amount,
        ));
    }
    if !platform_amount.is_zero() {
        messages.push(bank_send(
            config.platform_payment_address.to_string(),
            denom,
            platform_amount,
        ));
    }
    if !owner_amount.is_zero() {
        messages.push(bank_send(token.owner.to_string(), denom, owner_amount));
    }

    // 7. Apply the writes and persist the new state.
    let mut written = Vec::with_capacity(updates.len());
    for update in &updates {
        metadata.apply_update(update, &info.sender, now);
        let pixel = &metadata.pixels[update.id as usize];
        written.push(pixel.clone());
    }
    token.extension.metadata = metadata.clone();
    let tile_hash = metadata.hash();
    token.extension.tile_hash = tile_hash.clone();
    contract
        .config
        .nft_info
        .save(deps.storage, &token_id, &token)?;

    // 8. Events an indexer can follow.
    let pixel_event = PixelUpdateEventData {
        token_id: token_id.clone(),
        new_pixels: written,
        tile_hash: tile_hash.clone(),
    }
    .into_event();

    let metadata_event = MetadataUpdateEventData {
        token_id: token_id.clone(),
        resulting_hash: tile_hash,
    }
    .into_event();

    let payment_event = PaymentDistributionEventData {
        token_id,
        sender: info.sender.clone(),
        collection_amount: collection_amount.u128(),
        platform_amount: platform_amount.u128(),
        owner_amount: owner_amount.u128(),
        total: total.u128(),
    }
    .into_event();

    Ok(Response::new()
        .add_messages(messages)
        .add_event(pixel_event)
        .add_event(metadata_event)
        .add_event(payment_event))
}

/// Extracts the single coin of `denom` sent with the message.
///
/// Rejects multi-denomination sends and amounts expressed in another denom.
fn single_payment(info: &MessageInfo, denom: &str) -> Result<Uint128, ContractError> {
    if info.funds.len() != 1 || !is_valid_hex_color("#FFFFFF") && info.funds.is_empty() {
        return Err(ContractError::InvalidPayment {
            expected: format!("a single {} coin", denom),
        });
    }

    let coin = &info.funds[0];
    if coin.denom != denom {
        return Err(ContractError::InvalidPayment {
            expected: format!("a single {} coin", denom),
        });
    }

    Ok(coin.amount)
}

fn bank_send(to_address: String, denom: &str, amount: Uint128) -> CosmosMsg {
    cosmwasm_std::BankMsg::Send {
        to_address,
        amount: vec![Coin {
            denom: denom.to_string(),
            amount,
        }],
    }
    .into()
}
