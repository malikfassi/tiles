# Tiles NFT Design

## Message Handling Architecture

### Problem Statement
We need to extend `sg721-base` with custom functionality for pixel management while:
1. Maintaining compatibility with the Stargaze ecosystem (vending minter, etc.)
2. Supporting all base NFT operations
3. Adding custom message types for pixel management
4. Avoiding modifications to base contracts

### Considered Approaches

#### 3. Contract Wrapper with Delegation (Selected)
```rust
// Define our extension messages
#[cw_serde]
pub enum TileExecuteMsg {
    SetPixelColor { ... },
    UpdateConfig { ... },
}

// Use sg721_base's ExecuteMsg with our extension
pub type ExecuteMsg = sg721_base::msg::ExecuteMsg<Tile, TileExecuteMsg>;

// Wrapper contract
pub struct TilesContract<'a> {
    pub base: Sg721Contract<'a, Tile>,
}

// Example implementation with mint override
impl<'a> TilesContract<'a> {
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint { token_id, owner, token_uri, extension } => {
                // Set default metadata if none provided
                let extension = extension.unwrap_or_else(|| Tile {
                    tile_hash: TileMetadata::default().hash(),
                });
                
                // Forward to base contract with our extension
                self.base.mint(deps, env, info, NftParams::NftData {
                    token_id,
                    owner,
                    token_uri,
                    extension,
                })
            },
            ExecuteMsg::Extension { msg } => match msg {
                TileExecuteMsg::SetPixelColor { ... } => self.set_pixel_color(...),
                TileExecuteMsg::UpdateConfig { ... } => self.update_config(...),
            },
            // Forward all other messages to base contract
            base_msg => self.base.execute(deps, env, info, base_msg),
        }
    }
}
```

Benefits:
- Maintains ecosystem compatibility
- Properly handles both base and extension messages
- Clean separation of concerns
- No modification of base contracts needed

### Implementation Details

1. **Message Flow**
   - Base NFT messages (transfer, mint, etc.) -> Delegated to sg721-base
   - Extension messages (SetPixelColor, etc.) -> Handled by our contract
   - Mint messages -> Intercepted to set defaults, then delegated

2. **Contract Structure**
   - `TilesContract` wraps `Sg721Contract`
   - Delegates base functionality
   - Implements custom message handling
   - Overrides specific base messages when needed

3. **Type Safety**
   - Uses sg721-base's message types
   - Adds strongly-typed extension messages
   - Maintains type safety throughout

4. **Default Metadata Handling**
   - Intercepts mint messages
   - Sets default tile metadata if none provided
   - Ensures NFTs always have valid tile state
   - Maintains compatibility with vending minter

### Compatibility

1. **Vending Minter**
   - Works without modification
   - Only interacts with base NFT functionality
   - Unaffected by our extensions
   - Default metadata handling ensures valid NFTs

2. **Stargaze Ecosystem**
   - Fully compatible with sg721 standard
   - Supports all base NFT operations
   - Integrates with existing tools and UI

### Future Considerations

1. **Upgrades**
   - Base contract upgrades don't affect our extensions
   - Extension changes don't affect base functionality
   - Clean separation allows independent evolution

2. **Testing**
   - Can test base and extension functionality separately
   - Easier to maintain test coverage
   - Clear boundaries for test cases
   - Specific tests for default metadata handling 