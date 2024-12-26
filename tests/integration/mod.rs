use cosmwasm_std::Addr;
use crate::common::TestSetup;
use tiles::core::tile::{Tile, metadata::TileMetadata};

#[test]
fn test_full_minting_flow() -> anyhow::Result<()> {
    let mut setup = TestSetup::new()?;
    let user = Addr::unchecked("user");

    // Mint a tile for the user
    let token_id = setup.mint_token(&user)?;

    // Verify the tile ownership and metadata
    let tiles = setup.get_tiles_contract();
    
    // Create expected metadata and hash
    let metadata = TileMetadata::new();
    let expected_hash = Tile::generate_hash(&token_id, &metadata.pixels);

    // Query token info
    let token = tiles.query_token(&setup.app, &token_id)?;
    
    // Verify ownership and extension
    assert_eq!(token.owner, user.to_string());
    assert_eq!(token.extension.tile_hash, expected_hash);

    Ok(())
} 