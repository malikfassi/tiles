use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msg::{SetPixelColorMsg, UpdateConfigMsg};
use crate::state::{CONFIG, TileState, TILE_STATES};

pub fn execute_set_pixel_color(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SetPixelColorMsg,
) -> Result<Response, ContractError> {
    // Load config
    let _config = CONFIG.load(deps.storage)?;

    // Validate message size
    if msg.max_message_size > crate::state::MAX_MESSAGE_SIZE {
        return Err(ContractError::MessageTooLarge {});
    }

    let num_updates = msg.updates.len();

    // Process each tile update
    for update in msg.updates {
        // Load tile state
        let mut tile_state = TILE_STATES
            .load(deps.storage, &update.tile_id)
            .unwrap_or_default();

        // Convert metadata to state format
        let mut pixels = Vec::new();
        for p in update.current_metadata.pixels {
            pixels.push(crate::state::PixelData {
                id: p.id,
                color: p.color,
                expiration: p.expiration,
                last_updated_by: p.last_updated_by,
            });
        }
        let metadata = crate::state::TileMetadata {
            tile_id: update.current_metadata.tile_id,
            pixels,
        };

        // Verify current state
        tile_state.verify_metadata(&update.tile_id, &metadata)?;

        // Apply updates and generate new hash
        for pixel_update in update.updates.pixels {
            // Find pixel index or create new pixel
            let pixel_idx = tile_state.pixels
                .iter()
                .position(|p| p.id == pixel_update.id);

            match pixel_idx {
                Some(idx) => {
                    // Update existing pixel
                    let pixel = &mut tile_state.pixels[idx];
                    pixel.color = pixel_update.color;
                    pixel.expiration = pixel_update.expiration;
                    pixel.last_updated_by = info.sender.to_string();
                }
                None => {
                    // Create new pixel
                    tile_state.pixels.push(crate::state::PixelData {
                        id: pixel_update.id,
                        color: pixel_update.color,
                        expiration: pixel_update.expiration,
                        last_updated_by: info.sender.to_string(),
                    });
                }
            }
        }

        // Save new state
        tile_state.tile_hash = TileState::generate_hash(&update.tile_id, &tile_state.pixels);
        TILE_STATES.save(deps.storage, &update.tile_id, &tile_state)?;
    }

    Ok(Response::new()
        .add_attribute("action", "set_pixel_color")
        .add_attribute("num_updates", num_updates.to_string()))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only admin can update config
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Update config fields
    if let Some(dev_address) = msg.dev_address {
        config.dev_address = deps.api.addr_validate(&dev_address)?;
    }
    if let Some(dev_fee_percent) = msg.dev_fee_percent {
        config.dev_fee_percent = dev_fee_percent;
    }
    if let Some(base_price) = msg.base_price {
        config.base_price = base_price;
    }
    if let Some(price_scaling) = msg.price_scaling {
        config.price_scaling = Some(price_scaling);
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "update_config")
        .add_attribute("dev_address", config.dev_address)
        .add_attribute("dev_fee_percent", config.dev_fee_percent.to_string())
        .add_attribute("base_price", config.base_price.to_string()))
} 