# Tiles NFT Project - Implementation Specification

## 1. Contract Architecture

### 1.1. Dependencies & Setup
```toml
[package]
name = "tiles"
version = "0.1.0"
edition = "2021"
description = "A Stargaze NFT collection with customizable tile colors"
license = "Apache-2.0"
rust-version = "1.73.0"

[dependencies]
cosmwasm-schema = "1.5.0"
cosmwasm-std = "1.5.0"
cw-storage-plus = "1.2.0"
cw2 = "1.1.2"
cw721 = "0.18.0"
cw721-base = { version = "0.18.0", features = ["library"] }
schemars = "0.8.15"
serde = { version = "1.0.195", default-features = false, features = ["derive"] }
sg721 = "3.15.0"
sg721-base = "3.15.0"
sg2 = "3.15.0"
sg-std = "3.2.0"
thiserror = "1.0.56"
sha2 = { version = "0.10.8", default-features = false }
```

### 1.2. State Management
```rust
// Core contract structure inheriting from sg721-base
pub struct TilesContract<'a> {
    pub base: Sg721Contract<'a, Extension>, // Base sg721 contract
    pub config: Item<'a, Config>,  // Contract configuration
}

// Extension type for storing tile data in sg721 token
#[cw_serde]
pub struct Extension {
    pub tile_hash: String,  // Hash of current off-chain metadata
    pub pixels: Vec<PixelData>,  // Current pixel states
}

// Configuration (stored on-chain)
pub struct Config {
    pub admin: Addr,          // Contract admin
    pub minter: Addr,         // Minting contract address
    pub dev_address: Addr,    // Developer fee recipient
    pub dev_fee_percent: Decimal,  // Fee on pixel updates (e.g., 5%)
    pub base_price: Uint128,  // Base price per pixel
    pub price_scaling: Option<PriceScaling>,  // Price scaling parameters
}

// Price scaling configuration
pub struct PriceScaling {
    pub hour_1_price: Uint128,    // ≤1 hour price
    pub hour_12_price: Uint128,   // ≤12 hours price
    pub hour_24_price: Uint128,   // ≤24 hours price
    pub quadratic_base: Uint128,  // Base for >24 hours
}
```

### 1.3. Messages & Entry Points
```rust
// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Standard sg721 messages
    #[serde(flatten)]
    Base(sg721::ExecuteMsg<Extension>),
    
    // Custom messages
    SetPixelColor(SetPixelColorMsg),
    UpdateConfig(UpdateConfigMsg),
}

// Query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Standard sg721 queries
    #[serde(flatten)]
    Base(sg721::QueryMsg),
    
    // Custom queries
    Config {},
    TileState { token_id: String },
}

// Instantiate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // Standard sg721 fields (set by vending minter)
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub collection_info: CollectionInfo,
    
    // Our custom config
    pub dev_address: String,
    pub dev_fee_percent: Decimal,
    pub base_price: Uint128,
    pub price_scaling: Option<PriceScaling>,
}
```

### 1.4. Error Handling
```rust
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Base(#[from] sg721::ContractError),

    #[error("Hash mismatch - state has been modified")]
    HashMismatch {},

    #[error("Invalid pixel update")]
    InvalidPixelUpdate {},

    #[error("Invalid color format")]
    InvalidColorFormat {},

    #[error("Invalid expiration")]
    InvalidExpiration {},

    #[error("Message too large")]
    MessageTooLarge {},

    #[error("Unauthorized")]
    Unauthorized {},
}
```

## 2. Core Features

### 2.1. Pixel Management
```rust
// Off-chain metadata structure
pub struct TileMetadata {
    pub tile_id: String,
    pub pixels: Vec<PixelData>,
}

// Update-specific metadata structure (optimized size)
pub struct TileUpdates {
    pub pixels: Vec<PixelUpdate>,  // Only pixels being updated
}

pub struct PixelData {
    pub id: u32,               // Position within tile (0 to pixels_per_tile-1)
    pub color: String,         // Hex color (#RRGGBB)
    pub expiration: u64,       // Timestamp when pixel expires
    pub last_updated_by: Addr, // Address that last updated the pixel
}

pub struct PixelUpdate {
    pub id: u32,               // Position to update
    pub color: String,         // New color
    pub expiration: u64,       // New expiration
}

// Update message
pub struct SetPixelColorMsg {
    pub updates: Vec<TileUpdate>,    // Multiple tiles can be updated
    pub max_message_size: u32,       // Maximum message size in bytes
}

pub struct TileUpdate {
    pub tile_id: String,
    pub current_metadata: TileMetadata,  // For hash verification
    pub updates: TileUpdates,            // Only the pixels being changed
}
```

### 2.2. Constants & Validation
```rust
// Compile-time constants
pub const PIXELS_PER_TILE: u32 = 100;
pub const MAX_MESSAGE_SIZE: u32 = 128 * 1024;  // 128KB
pub const NATIVE_DENOM: &str = "ustars";
pub const MIN_EXPIRATION: u64 = 60;          // 1 minute
pub const MAX_EXPIRATION: u64 = 31_536_000;  // 1 year

// Validation functions
fn validate_color(color: &str) -> Result<(), ContractError> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ContractError::InvalidColorFormat {});
    }
    if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ContractError::InvalidColorFormat {});
    }
    Ok(())
}

fn validate_expiration(expiration: u64) -> Result<(), ContractError> {
    if expiration < MIN_EXPIRATION || expiration > MAX_EXPIRATION {
        return Err(ContractError::InvalidExpiration {});
    }
    Ok(())
}
```

### 2.3. Hash Verification
```rust
// Hash input structure
#[derive(Serialize)]
pub struct HashInput<'a> {
    pub tile_id: &'a str,
    pub pixels: &'a [PixelData],
}

impl TileState {
    // Generate hash from metadata
    pub fn generate_hash(tile_id: &str, pixels: &[PixelData]) -> String {
        let input = HashInput { tile_id, pixels };
        let hash = Sha256::new()
            .chain_update(to_vec(&input).unwrap())
            .finalize();
        hex::encode(hash)
    }

    // Verify current metadata matches stored hash
    pub fn verify_metadata(
        &self,
        tile_id: &str,
        metadata: &TileMetadata,
    ) -> Result<(), ContractError> {
        let current_hash = Self::generate_hash(tile_id, &metadata.pixels);
        if current_hash != self.tile_hash {
            return Err(ContractError::HashMismatch {});
        }
        Ok(())
    }
}
```

### 2.4. Fee Calculation
```rust
impl Config {
    // Calculate price based on expiration time
    pub fn calculate_pixel_price(&self, expiration_hours: u64) -> Uint128 {
        match expiration_hours {
            0..=1 => self.price_scaling.hour_1_price,
            2..=12 => self.price_scaling.hour_12_price,
            13..=24 => self.price_scaling.hour_24_price,
            _ => {
                // Quadratic scaling for >24h
                let seconds = expiration_hours * 3600;
                self.price_scaling.quadratic_base + 
                    Uint128::from(seconds).pow(2) / Uint128::from(1_000_000u128)
            }
        }
    }

    // Calculate total fees for updates
    pub fn calculate_fees(
        &self,
        updates: &TileUpdates,
    ) -> StdResult<(Uint128, Uint128)> {
        let mut total_amount = Uint128::zero();

        // Sum up price for each pixel update
        for update in &updates.pixels {
            let hours = (update.expiration - MIN_EXPIRATION) / 3600;
            total_amount += self.calculate_pixel_price(hours);
        }

        // Calculate fee split
        let dev_fee = total_amount * self.dev_fee_percent;
        let owner_payment = total_amount - dev_fee;

        Ok((dev_fee, owner_payment))
    }
}
```

## 3. Integration with sg721-base

The contract is now properly integrated with sg721-base, inheriting its NFT functionality while extending it with tile-specific features:

### 3.1. State Management
- The contract inherits from `Sg721Contract<Extension>` to leverage standard NFT functionality
- Tile data is stored directly in the NFT token's extension, providing better data cohesion
- Configuration and custom state are managed separately from the base contract

### 3.2. Message Handling
- Base NFT messages (mint, transfer, etc.) are handled through the sg721 implementation
- Custom messages for tile updates are added while maintaining compatibility
- Query messages support both standard NFT queries and tile-specific data

### 3.3. Error Handling
- Errors from the base contract are properly propagated and converted
- Custom errors for tile operations are maintained
- All errors follow the standard CosmWasm error pattern

### 3.4. Benefits of Integration
1. **Standard Compliance**: Full compatibility with Stargaze NFT standards
2. **Efficient Storage**: Tile data is directly associated with NFT tokens
3. **Type Safety**: Extension type ensures proper data handling
4. **Maintainability**: Clear separation between base and custom functionality
5. **Ecosystem Integration**: Works seamlessly with Stargaze tools and interfaces

This integration ensures that our contract maintains compatibility with the Stargaze ecosystem while adding the custom functionality needed for tile management.