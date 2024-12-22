use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response};
use sg721::ExecuteMsg as Sg721ExecuteMsg;
use sg_std::StargazeMsgWrapper;
use sg721_base::Sg721Contract;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, SetPixelColorMsg, UpdateConfigMsg};
use crate::state::{CONFIG, PIXELS_PER_TILE, PixelData, Extension};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Sg721Base(base_msg) => {
            let config = CONFIG.load(deps.storage)?;
            if info.sender != config.minter {
                return Err(ContractError::Unauthorized {});
            }
            match base_msg {
                Sg721ExecuteMsg::Mint { token_id, owner, token_uri, extension: _ } => {
                    let owner_addr = deps.api.addr_validate(&owner)?;
                    let creation_time = env.block.time.seconds();
                    
                    // Create default pixels (all white, expired)
                    let pixels: Vec<PixelData> = (0..PIXELS_PER_TILE)
                        .map(|id| PixelData::new_at_mint(id, owner_addr.clone(), creation_time))
                        .collect();

                    // Initialize extension with default pixels
                    let extension = Extension {
                        tile_hash: Extension::generate_hash(&token_id, &pixels),
                    };

                    // Forward mint message to sg721 with our extension
                    let mint_msg = Sg721ExecuteMsg::Mint {
                        token_id,
                        owner,
                        token_uri,
                        extension,
                    };
                    let contract: Sg721Contract<Extension> = Sg721Contract::default();
                    Ok(contract.execute(deps, env, info, mint_msg)?)
                }
                _ => {
                    let contract: Sg721Contract<Extension> = Sg721Contract::default();
                    Ok(contract.execute(deps, env, info, base_msg)?)
                }
            }
        },
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
    // Verify message size
    if msg.max_message_size > 128 * 1024 {
        return Err(ContractError::MessageTooLarge {});
    }

    let contract: Sg721Contract<Extension> = Sg721Contract::default();

    // Process each tile update
    for update in msg.updates {
        // Load token info to get extension
        let mut token = contract.parent.tokens.load(deps.storage, &update.tile_id)?;

        // Verify current state
        token.extension.verify_metadata(&update.tile_id, &update.current_metadata)?;

        // Apply updates and generate new hash
        let new_hash = token.extension.apply_updates(
            &update.current_metadata,
            &update.updates,
            &info.sender,
            env.block.time.seconds(),
        )?;

        // Save new extension
        token.extension.tile_hash = new_hash;
        contract.parent.tokens.save(deps.storage, &update.tile_id, &token)?;
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