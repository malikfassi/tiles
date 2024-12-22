use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721::ExecuteMsg as Sg721ExecuteMsg;
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;

use crate::defaults::constants::PIXELS_PER_TILE;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, SetPixelColorMsg, UpdateConfigMsg};
use crate::state::{Extension, PixelData, CONFIG};
use crate::validate;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension: _,
        } => {
            let config = CONFIG.load(deps.storage)?;
            if info.sender != config.minter {
                return Err(ContractError::Unauthorized {});
            }
            let owner_addr = deps.api.addr_validate(&owner)?;
            let creation_time = env.block.time.seconds();

            // Create default pixels (all white, expired)
            let pixels: Vec<PixelData> = (0..PIXELS_PER_TILE)
                .map(|id| PixelData::new_at_mint(id, owner_addr.clone(), creation_time))
                .collect();

            // Initialize extension with just the hash
            let extension = Extension {
                tile_hash: Extension::generate_hash(&token_id, &pixels),
            };

            // Forward mint message to sg721 with our extension
            let contract: Sg721Contract<Extension> = Sg721Contract::default();
            let mint_msg = Sg721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            };
            Ok(contract.execute(deps, env, info, mint_msg)?)
        }
        ExecuteMsg::SetPixelColor(msg) => execute_set_pixel_color(deps, env, info, msg),
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
    }
}

pub fn execute_set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SetPixelColorMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Validate message size
    validate::validate_message_size(&msg)?;

    let contract: Sg721Contract<Extension> = Sg721Contract::default();

    // Process each tile update
    for update in msg.updates {
        // Load token info to get extension
        let mut token = contract.parent.tokens.load(deps.storage, &update.tile_id)?;

        // Verify current state matches what client thinks it is
        token
            .extension
            .verify_metadata(&update.tile_id, &update.current_metadata)?;

        // Apply pixel updates
        let mut pixels = update.current_metadata.pixels;
        for pixel_update in update.updates.pixels {
            // Validate pixel update
            validate::validate_pixel_update(
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
            } else {
                return Err(ContractError::InvalidPixelUpdate {});
            }
        }

        // Generate new hash from updated pixels
        let new_hash = Extension::generate_hash(&update.tile_id, &pixels);

        // Save updated extension with just the hash
        token.extension = Extension {
            tile_hash: new_hash,
        };
        contract
            .parent
            .tokens
            .save(deps.storage, &update.tile_id, &token)?;
    }

    Ok(Response::new().add_attribute("action", "set_pixel_color"))
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

    // Validate config update
    validate::validate_config_update(&msg)?;

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

    Ok(Response::new().add_attribute("action", "update_config"))
}
