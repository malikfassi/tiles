# Tiles NFT Project Codebase Analysis

## Project Structure 

## File Analysis

### src/lib.rs
**Description**: Main contract implementation file containing core NFT tile functionality
**Scope**: Contract implementation
**Objects**:
- Structs:
  - `Contract` - Main contract struct
  - `TileState` - On-chain tile state
  - `Config` - Contract configuration
  - `PriceScaling` - Price scaling parameters
- Messages:
  - `ExecuteMsg` - Contract execution messages
  - `QueryMsg` - Contract query messages
  - `InstantiateMsg` - Contract initialization
- Functions:
  - `execute_set_pixel_color` - Main pixel update logic
  - `apply_updates` - Helper for applying pixel updates
  - `validate_color` - Color format validation
  - `validate_expiration` - Expiration time validation

### tests/mod.rs
**Description**: Root test module organizing test structure
**Scope**: Test organization
**Objects**:
- Modules:
  - `common` - Common test utilities
  - `unit` - Unit test cases

### tests/common/mod.rs
**Description**: Common test utilities module definition
**Scope**: Test utilities
**Objects**:
- Modules:
  - `mock` - Mock data generation
  - `tiles_contract` - Contract test helpers
  - `vending_factory` - Factory integration helpers

### tests/common/mock.rs
**Description**: Mock data generation for testing
**Scope**: Test data
**Objects**:
- Functions:
  - `mock_config` - Generate test configuration
  - `mock_tile_state` - Generate test tile state
  - `mock_pixel_updates` - Generate test pixel updates

### tests/common/tiles_contract.rs
**Description**: Test helpers for tiles contract interaction
**Scope**: Contract testing
**Objects**:
- Structs:
  - `TilesContract` - Test contract wrapper
- Functions:
  - `instantiate_tiles` - Test contract instantiation
  - `execute_update` - Test pixel updates
  - `query_state` - Test state queries

### tests/common/vending_factory.rs
**Description**: Test helpers for factory integration
**Scope**: Factory integration testing
**Objects**:
- Structs:
  - `VendingFactory` - Factory test wrapper
- Functions:
  - `deploy_collection` - Test collection deployment
  - `mint_tile` - Test tile minting

### tests/unit/mod.rs
**Description**: Unit test module organization
**Scope**: Unit tests
**Objects**:
- Modules:
  - `hash_tests` - Hash verification tests

### tests/unit/hash_tests.rs
**Description**: Tests for hash verification functionality
**Scope**: Hash verification testing
**Objects**:
- Test Functions:
  - `test_hash_generation`
  - `test_hash_verification`
  - `test_hash_mismatch`

### Cargo.toml
**Description**: Project configuration and dependencies
**Scope**: Project setup
**Objects**:
- Dependencies:
  - cosmwasm-std
  - cw-storage-plus
  - sg721
  - Other core dependencies
- Features:
  - backtraces
  - library

### .gitignore
**Description**: Git ignore configuration
**Scope**: Version control
**Objects**:
- Ignore patterns:
  - Build artifacts
  - IDE files
  - System files
  - Dependencies

This analysis provides a comprehensive overview of the project structure and the purpose/content of each file. The codebase follows a clean separation of concerns between contract implementation, testing utilities, and integration tests.