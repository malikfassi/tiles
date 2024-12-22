use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::error::ContractError;
use crate::msg::{SetPixelColorMsg, UpdateConfigMsg};
use crate::state::CONFIG;
use crate::types::Extension;
use crate::utils::{
    extension, hash, payment, price, validation,
    pixel_update_attributes, config_update_attributes, create_event_response,
};

pub fn execute_set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SetPixelColorMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Load config
    let config = CONFIG.load(deps.storage)?;

    // Validate message size
    validation::validate_message_size(&msg)?;

    let mut contract: Sg721Contract<Extension> = Sg721Contract::default();
    let mut total_pixels_updated = 0u32;
    let updates_len = msg.updates.len();
    let mut response = Response::new();

    // Process each tile update
    for update in msg.updates {
        // Load and verify token extension
        extension::load_and_verify_token(&contract, &update.tile_id, &update.current_metadata)?;

        // Validate the current metadata structure
        validation::validate_tile_metadata(&update.current_metadata)?;

        // Calculate fees for this update
        let fees = price::calculate_batch_fees(
            &config,
            &update.updates.pixels,
            env.block.time.seconds(),
        )?;

        // Process payment and get payment info
        let payment_info = payment::process_payment(&info, fees.total_amount, &config)?;

        // Add payment messages to response
        response = payment::add_payment_messages(response, payment_info);

        // Apply pixel updates
        let mut pixels = update.current_metadata.pixels;
        for pixel_update in update.updates.pixels {
            // Find pixel to update
            let pixel = pixels.iter().find(|p| p.id == pixel_update.id)
                .ok_or_else(|| ContractError::InvalidPixelUpdate {
                    id: pixel_update.id,
                    max: pixels.len() as u32 - 1,
                })?;

            // Check if pixel is expired
            let current_time = env.block.time.seconds();
            if current_time < pixel.expiration {
                return Err(ContractError::PixelNotExpired {
                    current: current_time,
                    expiration: pixel.expiration,
                });
            }

            // Validate pixel update
            validation::validate_pixel_update(
                pixel_update.id,
                &pixel_update.color,
                pixel_update.expiration,
                &env,
            )?;

            // Update pixel
            if let Some(pixel) = pixels.iter_mut().find(|p| p.id == pixel_update.id) {
                pixel.color = pixel_update.color;
                pixel.expiration = pixel_update.expiration;
                pixel.last_updated_by = info.sender.clone();
                pixel.last_updated_at = current_time;
                total_pixels_updated += 1;
            }
        }

        // Update token extension with new pixel data
        extension::update_token_extension(&mut contract, &update.tile_id, &pixels)?;
    }

    // Add pixel update event attributes
    Ok(response.add_attributes(pixel_update_attributes(
        updates_len,
        total_pixels_updated,
        &info.sender,
    )))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only admin can update config
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Track changes for response
    let mut updated_fields = Vec::new();

    // Update config fields
    if let Some(tiles_royalty_payment_address) = msg.tiles_royalty_payment_address {
        let new_royalty_address = deps.api.addr_validate(&tiles_royalty_payment_address)?;
        if new_royalty_address != config.tiles_royalty_payment_address {
            config.tiles_royalty_payment_address = new_royalty_address;
            updated_fields.push("tiles_royalty_payment_address");
        }
    }
    if let Some(tiles_royalties) = msg.tiles_royalties {
        if tiles_royalties != config.tiles_royalties {
            config.tiles_royalties = tiles_royalties;
            updated_fields.push("tiles_royalties");
        }
    }
    if let Some(price_scaling) = msg.price_scaling {
        // Validate new price scaling
        price::validate_price_scaling(&price_scaling)?;
        config.price_scaling = price_scaling;
        updated_fields.push("price_scaling");
    }

    CONFIG.save(deps.storage, &config)?;

    // Create config update event response
    Ok(Response::new().add_attributes(config_update_attributes(&updated_fields)))
}
