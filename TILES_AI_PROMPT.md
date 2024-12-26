# Tiles NFT Project - Smart Contract Specification

## Base Architecture
- Inherits directly from `sg721_base::Sg721Contract` to ensure all features are available by default
- Uses `StargazeMsgWrapper` for message handling
- Implements `Tile` extension for NFT metadata
- Maintains hash-based state verification

## Core Modules

### 1. Core Types (`core/`)
Core domain logic and data structures for the Tiles NFT project.

```rust
use sg721_base::Sg721Contract;
use sg_std::StargazeMsgWrapper;
use sg721::InstantiateMsg as Sg721InstantiateMsg;
use sg721::CollectionInfo;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, StdResult};
```

#### Tile Module (`core/tile/`)
Handles NFT state verification and metadata management:
- `Tile`: Extension storing the hash of current pixel state
- `TileMetadata`: Off-chain metadata structure for pixel data
- `PixelData`: Individual pixel state with ownership and expiration

```rust
// core/tile/mod.rs
#[cw_serde]
pub struct Tile {
    pub tile_hash: String,  // SHA-256 hash of current state
}

// core/tile/metadata.rs
#[cw_serde]
pub struct TileMetadata {
    pub pixels: Vec<PixelData>,
}

#[cw_serde]
pub struct PixelData {
    pub id: u32,               // 0-99
    pub color: String,         // #RRGGBB
    pub expiration: u64,       // Unix timestamp
    pub last_updated_by: Addr, // Last modifier
    pub last_updated_at: u64,  // Unix timestamp of last update
}
```

#### Pricing Module (`core/pricing/`)
Handles dynamic pricing calculations for pixel updates:
- Scales price based on expiration time
- Provides quadratic pricing for multiple updates

```rust
// core/pricing/mod.rs
pub struct PriceScaling<T> {
    pub hour_1_price: Uint128,    // Default: 100_000_000
    pub hour_12_price: Uint128,   // Default: 200_000_000
    pub hour_24_price: Uint128,   // Default: 300_000_000
    pub quadratic_base: Uint128,  // Default: 400_000_000
}
```

### 2. Contract State (`contract/state.rs`)
Main contract implementation inheriting from sg721_base:
- `TilesContract`: Main contract struct with sg721_base inheritance
- `Config`: Contract configuration and pricing parameters

```rust
// contract/state.rs
pub type TilesContract<'a> = Sg721Contract<'a, Tile, StargazeMsgWrapper>;

#[cw_serde]
pub struct Config {
    pub dev_address: Addr,    // Set to creator
    pub dev_fee_percent: Decimal,  // Default: 5%
    pub price_scaling: PriceScaling<Decimal>, // Default scaling values
}
```

### 3. Message Types (`contract/msg.rs`)
Contract interface defining all supported operations:
- Inherits all sg721 NFT operations by default
- Adds custom messages for pixel updates and config management

```rust
// contract/msg.rs
pub type InstantiateMsg = Sg721InstantiateMsg<T>;

#[cw_serde]
pub enum ExecuteMsg {
    // All sg721 messages are available by default
    SetPixelColor {
        token_id: String,
        current_metadata: TileMetadata,
        updates: Vec<PixelData>,
    },
    UpdateConfig {
        dev_address: Option<String>,
        dev_fee_percent: Option<Decimal>,
        price_scaling: Option<PriceScaling<Decimal>>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    // All sg721 queries are available by default
    Config {},
}
```

### 4. Default Values (`defaults/`)
Default configuration and constants:
- config : all default values for tile contract config, instanciate, execute, vending instantiate etc ... 
- constants : all static constants for the contract protocol 

## Critical Patterns

### State Management
1. Always verify current hash before updates
2. Update both NFT and pixel state atomically
3. Compute new hash after changes
4. Emit detailed events

### Message Flow
1. NFT operations: Handled directly by sg721 inheritance
2. Custom operations: Implemented in execute handler
3. Queries: Inherit sg721 queries + custom queries
4. Responses: Maintain sg721 compatibility

### Testing Requirements
1. Use vending-factory for collection creation
2. Use vending-minter for test minting
3. Use sg-multi-test for contract testing
