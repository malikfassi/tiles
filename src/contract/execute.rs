use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Decimal};
use sg_std::StargazeMsgWrapper;
use sg721::ExecuteMsg as Sg721ExecuteMsg;
use sg721_base::ContractError;

use crate::{
    contract::{
        contract::Contract,
        msg::ExecuteMsg,
        state::{CONFIG, PriceScaling},
    },
    core::{
        tile::{
            metadata::TileMetadata,
            Tile,
        },
        pricing::{
            calculation::calculate_pixel_price,
            PriceScaling as CorePriceScaling,
        },
    },
    defaults::tile::DEFAULT_TILE_METADATA,
};

pub fn execute_handler(
    contract: &Contract,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::SetPixelColor { token_id, position, color, expiration } => {
            execute_set_pixel_color(contract, deps, env, info, token_id, position, color, expiration)
        }
        ExecuteMsg::UpdateConfig { dev_address, dev_fee_percent, base_price, price_scaling } => {
            execute_update_config(deps, info, dev_address, dev_fee_percent, base_price, price_scaling)
        }
        ExecuteMsg::Sg721(msg) => match msg {
            Sg721ExecuteMsg::Mint { token_id, owner, token_uri, extension: _ } => {
                // Create default tile metadata
                let metadata = DEFAULT_TILE_METADATA.clone();
                
                // Generate tile hash
                let tile_hash = Tile::generate_hash(&token_id, &metadata.pixels);

                // Save metadata
                deps.storage.set(
                    format!("tile_metadata:{}", token_id).as_bytes(),
                    &cosmwasm_std::to_json_vec(&metadata)?,
                );

                // Create extension and call parent mint
                let res = contract.base.execute(deps, env, info, Sg721ExecuteMsg::Mint { 
                    token_id,
                    owner,
                    token_uri,
                    extension: Tile { tile_hash: tile_hash.clone() },
                })?;

                Ok(Response::new()
                    .add_attributes(res.attributes)
                    .add_events(res.events)
                    .add_attribute("action", "mint")
                    .add_attribute("tile_hash", tile_hash))
            }
            msg => {
                let res = contract.base.execute(deps, env, info, msg)?;
                Ok(Response::new()
                    .add_attributes(res.attributes)
                    .add_events(res.events))
            }
        }
    }
}

pub fn execute_set_pixel_color(
    contract: &Contract,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    position: u32,
    color: String,
    expiration: u64,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Load config for pricing
    let config = CONFIG.load(deps.storage)?;
    
    // Convert price scaling and calculate price
    let core_scaling = CorePriceScaling {
        hour_1_price: Decimal::from_ratio(config.price_scaling.hour_1_price, 1u128),
        hour_12_price: Decimal::from_ratio(config.price_scaling.hour_12_price, 1u128),
        hour_24_price: Decimal::from_ratio(config.price_scaling.hour_24_price, 1u128),
        quadratic_base: Decimal::from_ratio(config.price_scaling.quadratic_base, 1u128),
    };
    let price = calculate_pixel_price(&core_scaling, expiration)
        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::generic_err(e.to_string())))?;
    
    // Verify payment in ustars
    if info.funds.is_empty() || 
       info.funds[0].denom != "ustars" || 
       info.funds[0].amount < price {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Insufficient funds")));
    }

    // Load token info and extension
    let mut token = contract.base.tokens.load(deps.storage, &token_id)?;
    
    // Load metadata from state
    let mut metadata = TileMetadata::new();
    if !token.extension.tile_hash.is_empty() {
        // If tile exists, verify and load metadata from state
        let state = deps.storage.get(format!("tile_metadata:{}", token_id).as_bytes())
            .ok_or_else(|| ContractError::Std(cosmwasm_std::StdError::not_found("Tile metadata")))?;
        metadata = cosmwasm_std::from_json(&state)?;
        token.extension.verify_metadata(&token_id, &metadata)
            .map_err(|e| ContractError::Std(cosmwasm_std::StdError::generic_err(e.to_string())))?;
    }
    
    // Check if pixel is expired
    if let Ok(pixel) = metadata.get_pixel(position) {
        if pixel.expiration > env.block.time.seconds() {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Pixel not expired")));
        }
    }

    // Update pixel
    metadata.update_pixel(position, color.clone(), expiration, info.sender.clone())
        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::generic_err(e.to_string())))?;
    
    // Update tile hash
    let new_tile = Tile {
        tile_hash: Tile::generate_hash(&token_id, &metadata.pixels),
    };
    
    // Save updated state
    token.extension = new_tile;
    contract.base.tokens.save(deps.storage, &token_id, &token)?;
    
    // Save metadata separately
    deps.storage.set(
        format!("tile_metadata:{}", token_id).as_bytes(),
        &cosmwasm_std::to_json_vec(&metadata)?,
    );

    Ok(Response::new()
        .add_attribute("action", "set_pixel_color")
        .add_attribute("token_id", token_id)
        .add_attribute("position", position.to_string())
        .add_attribute("color", color)
        .add_attribute("expiration", expiration.to_string())
        .add_attribute("price", price.to_string()))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    dev_address: Option<String>,
    dev_fee_percent: Option<u64>,
    base_price: Option<u128>,
    price_scaling: Option<PriceScaling>,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    
    // Only admin can update config
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update fields if provided
    if let Some(addr) = dev_address {
        config.dev_address = deps.api.addr_validate(&addr)?;
    }
    if let Some(fee) = dev_fee_percent {
        config.dev_fee_percent = fee;
    }
    if let Some(price) = base_price {
        config.base_price = price;
    }
    if let Some(scaling) = price_scaling {
        config.price_scaling = scaling;
    }
    
    // Save updated config
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new().add_attribute("action", "update_config"))
}
