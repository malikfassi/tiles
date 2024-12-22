use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::error::ContractError;
use crate::msg::{SetPixelColorMsg, UpdateConfigMsg};
use crate::state::CONFIG;
use crate::types::Extension;
use crate::utils::{hash, price, validation};

pub fn execute_set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SetPixelColorMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Validate message size
    validation::validate_message_size(&msg)?;

    let contract: Sg721Contract<Extension> = Sg721Contract::default();
    let mut total_pixels_updated = 0u32;
    let updates_len = msg.updates.len();

    // Process each tile update
    for update in msg.updates {
        // Load token info to get extension
        let mut token = contract.parent.tokens.load(deps.storage, &update.tile_id)?;

        // Verify current state matches what client thinks it is
        hash::verify_metadata(
            &token.extension,
            &update.tile_id,
            &update.current_metadata,
        )?;

        // Validate the current metadata structure
        validation::validate_tile_metadata(&update.current_metadata)?;

        // Apply pixel updates
        let mut pixels = update.current_metadata.pixels;
        for pixel_update in update.updates.pixels {
            // Validate pixel update
            validation::validate_pixel_update(
                pixel_update.id,
                &pixel_update.color,
                pixel_update.expiration,
                &env,
            )?;

            // Find and update pixel
            if let Some(pixel) = pixels.iter_mut().find(|p| p.id == pixel_update.id) {
                pixel.color = pixel_update.color;
                pixel.expiration = pixel_update.expiration;
                pixel.last_updated_by = info.sender.clone();
                pixel.last_updated_at = env.block.time.seconds();
                total_pixels_updated += 1;
            } else {
                return Err(ContractError::InvalidPixelUpdate {
                    id: pixel_update.id,
                    max: pixels.len() as u32 - 1,
                });
            }
        }

        // Generate new hash from updated pixels
        let new_hash = hash::generate_tile_hash(&update.tile_id, &pixels);

        // Save updated extension with just the hash
        token.extension = Extension {
            tile_hash: new_hash,
        };
        contract
            .parent
            .tokens
            .save(deps.storage, &update.tile_id, &token)?;
    }

    Ok(Response::new()
        .add_attributes(vec![
            ("action", "set_pixel_color"),
            ("tiles_updated", &updates_len.to_string()),
            ("pixels_updated", &total_pixels_updated.to_string()),
            ("updater", info.sender.as_str()),
        ]))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only minter can update config
    if info.sender != config.minter {
        return Err(ContractError::Unauthorized {});
    }

    // Track changes for response
    let mut updated_fields = Vec::new();

    // Update config fields
    if let Some(dev_address) = msg.dev_address {
        let new_dev_address = deps.api.addr_validate(&dev_address)?;
        if new_dev_address != config.dev_address {
            config.dev_address = new_dev_address;
            updated_fields.push("dev_address");
        }
    }
    if let Some(dev_fee_percent) = msg.dev_fee_percent {
        if dev_fee_percent != config.dev_fee_percent {
            config.dev_fee_percent = dev_fee_percent;
            updated_fields.push("dev_fee_percent");
        }
    }
    if let Some(base_price) = msg.base_price {
        if base_price != config.base_price {
            config.base_price = base_price;
            updated_fields.push("base_price");
        }
    }
    if let Some(price_scaling) = msg.price_scaling {
        // Validate new price scaling
        price::validate_price_scaling(&price_scaling)?;
        config.price_scaling = price_scaling;
        updated_fields.push("price_scaling");
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "update_config"),
        ("updated_fields", &updated_fields.join(",")),
        ("admin", info.sender.as_str()),
    ]))
}
