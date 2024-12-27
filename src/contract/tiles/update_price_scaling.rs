use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg721_base::Sg721Contract;
use sg721::RoyaltyInfoResponse;
use sg_std::StargazeMsgWrapper;

use crate::contract::error::ContractError;
use crate::contract::state::PRICE_SCALING;
use crate::core::pricing::PriceScaling;
use crate::core::tile::Tile;

pub fn update_price_scaling(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_price_scaling: PriceScaling,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    println!("\n=== Update Price Scaling Execution ===");
    println!("Sender: {}", info.sender);
    println!("New price scaling: {:#?}", new_price_scaling);

    // Get collection info from contract
    println!("Loading collection info...");
    let contract = Sg721Contract::<Tile>::default();
    let collection_info = contract.collection_info.load(deps.storage)?;
    println!("Collection info: {:#?}", collection_info);

    // Only royalty payment address can update prices
    if let Some(royalty_info) = collection_info.royalty_info {
        println!("Royalty payment address: {}", royalty_info.payment_address);
        println!("Comparing with sender: {}", info.sender);

        if info.sender != royalty_info.payment_address {
            println!("❌ Unauthorized: sender is not royalty payment address");
            return Err(ContractError::Unauthorized {});
        }
        println!("✅ Sender is authorized");
    } else {
        println!("❌ No royalty info found in collection info");
        return Err(ContractError::Unauthorized {});
    }

    // Validate new price scaling
    println!("Validating new price scaling...");
    match new_price_scaling.validate() {
        Ok(_) => println!("✅ Price scaling validation passed"),
        Err(e) => {
            println!("❌ Price scaling validation failed: {}", e);
            return Err(ContractError::InvalidConfig(e.to_string()));
        }
    }

    // Save updated price scaling
    println!("Saving new price scaling to storage...");
    match PRICE_SCALING.save(deps.storage, &new_price_scaling) {
        Ok(_) => println!("✅ Price scaling saved successfully"),
        Err(e) => {
            println!("❌ Failed to save price scaling: {}", e);
            return Err(ContractError::InvalidConfig(e.to_string()));
        }
    }

    println!("=== Update Price Scaling Complete ===\n");

    Ok(Response::new()
        .add_attribute("action", "update_price_scaling")
        .add_attribute("sender", info.sender))
}
