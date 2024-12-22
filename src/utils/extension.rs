use cosmwasm_std::DepsMut;
use sg721_base::Sg721Contract;

use crate::error::ContractError;
use crate::types::{Extension, PixelData, TileMetadata};
use crate::utils::hash::generate_tile_hash;

/// Update token extension with new pixel data
pub fn update_token_extension(
    deps: DepsMut,
    contract: &mut Sg721Contract<Extension>,
    token_id: &str,
    pixels: &[PixelData],
) -> Result<(), ContractError> {
    // Generate new hash
    let new_hash = generate_tile_hash(token_id, pixels);

    // Update token extension
    let mut token = contract.parent.tokens.load(deps.storage, token_id)?;
    token.extension.tile_hash = new_hash;
    contract.parent.tokens.save(deps.storage, token_id, &token)?;

    Ok(())
}

/// Load and verify token extension
pub fn load_and_verify_token(
    deps: DepsMut,
    contract: &Sg721Contract<Extension>,
    token_id: &str,
    metadata: &TileMetadata,
) -> Result<(), ContractError> {
    // Load token
    let token = contract.parent.tokens.load(deps.storage, token_id)?;

    // Verify metadata matches stored hash
    let current_hash = generate_tile_hash(token_id, &metadata.pixels);
    if current_hash != token.extension.tile_hash {
        return Err(ContractError::HashMismatch {});
    }

    Ok(())
} 