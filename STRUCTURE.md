# Tiles NFT Project - Structure Analysis

## 1. Dependency Hierarchy

```
sg721-base (Our parent contract)
├── cw721-base (Base NFT implementation)
│   ├── cw721 (NFT standard interface)
│   └── cw-storage-plus (Storage primitives)
├── sg721 (Stargaze NFT interface)
└── sg-std (Stargaze standard utilities)
```

## 2. Contract Structure

### 2.1. Base Contract (sg721-base)
```rust
// From vendor/sg721-base/src/state.rs
pub struct Sg721Contract<'a, T> {
    pub parent: Parent<'a, T>,  // cw721-base contract
    pub collection_info: Item<'a, CollectionInfo<RoyaltyInfo>>,
    pub frozen_collection_info: Item<'a, bool>,
    pub royalty_updated_at: Item<'a, Timestamp>,
}

type Parent<'a, T> = cw721_base::Cw721Contract<'a, T, StargazeMsgWrapper, Empty, Empty>;
```

### 2.2. Our Contract (from ANALYSIS.md)
```rust
pub struct Contract<'a> {
    pub sg721_base: Sg721Contract<'a>,
    pub tile_states: Map<'a, &'a str, TileState>,
    pub config: Item<'a, Config>,
}
```

## 3. Message Types

### 3.1. Execute Messages
```rust
// sg721 base execute messages
pub enum ExecuteMsg<T, E> {
    TransferNft { recipient: String, token_id: String },
    SendNft { contract: String, token_id: String, msg: Binary },
    Approve { spender: String, token_id: String, expires: Option<Expiration> },
    Revoke { spender: String, token_id: String },
    ApproveAll { operator: String, expires: Option<Expiration> },
    RevokeAll { operator: String },
    Mint { token_id: String, owner: String, token_uri: Option<String>, extension: T },
    Burn { token_id: String },
    Extension { msg: E },
}

// Our execute messages
pub enum ExecuteMsg {
    Base(sg721_base::ExecuteMsg),  // Forward base NFT operations
    SetPixelColor(SetPixelColorMsg),
    UpdateConfig(UpdateConfigMsg),
}
```

### 3.2. Query Messages
```rust
// sg721 base query messages
pub enum QueryMsg {
    OwnerOf { token_id: String },
    Approval { token_id: String, spender: String },
    Approvals { token_id: String },
    AllOperators { owner: String },
    NumTokens {},
    ContractInfo {},
    NftInfo { token_id: String },
    AllNftInfo { token_id: String },
    Tokens { owner: String },
    AllTokens {},
    Minter {},
    CollectionInfo {},
}

// Our query messages
pub enum QueryMsg {
    Base(sg721_base::QueryMsg),
    Config {},
    TileState { token_id: String },
}
```

## 4. Storage Structure

### 4.1. NFT Token Storage (inherited)
```rust
pub struct TokenInfo<T> {
    pub owner: Addr,
    pub approvals: Vec<Approval>,
    pub token_uri: Option<String>,
    pub extension: T,  // Our tile extension
}

// Our extension
pub struct Extension {
    pub tile_hash: String,  // Hash of current off-chain metadata
}
```

### 4.2. Tile State Storage
```rust
pub struct TileState {
    pub tile_hash: String,  // Hash of current off-chain metadata
}

pub struct TileMetadata {
    pub tile_id: String,
    pub pixels: Vec<PixelData>,
}

pub struct PixelData {
    pub id: u32,
    pub color: String,
    pub expiration: u64,
    pub last_updated_by: Addr,
}
```

### 4.3. Configuration Storage
```rust
pub struct Config {
    pub admin: Addr,
    pub minter: Addr,
    pub dev_address: Addr,
    pub dev_fee_percent: Decimal,
    pub base_price: Uint128,
    pub price_scaling: PriceScaling,
}
```

## 5. Entry Points

### 5.1. Base Contract Entry Points
```rust
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<Response, ContractError>;
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError>;
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary>;
```

### 5.2. Our Entry Points
```rust
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<Response, ContractError>;
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError>;
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary>;
```

## 6. Key Implementation Notes

1. Message Handling:
   - Base NFT operations are forwarded to sg721-base
   - Custom pixel operations are handled by our contract
   - Config updates require admin authorization

2. State Management:
   - NFT state is managed by the parent contract
   - Tile state is managed in our extension
   - Config is managed separately for easy updates

3. Validation:
   - Color format validation (#RRGGBB)
   - Expiration time validation (1 min to 1 year)
   - Hash verification for state consistency

4. Error Handling:
   - Standard errors are propagated from base contract
   - Custom errors for pixel operations
   - Proper error conversion between contract types 