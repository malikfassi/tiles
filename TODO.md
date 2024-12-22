# Tiles Implementation TODO List

## 1. Contract Core (Priority: High)

### 1.1. Project Setup
- [x] Create project structure:
  ```
  tiles/
  ├── src/
  │   ├── lib.rs       # Contract entry points
  │   ├── state.rs     # State management
  │   ├── msg.rs       # Message definitions
  │   ├── error.rs     # Error handling
  │   ├── execute.rs   # Execute handlers
  │   └── query.rs     # Query handlers
  ├── Cargo.toml
  └── .cargo/config
  ```
- [x] Set up dependencies in Cargo.toml

### 1.2. Base Contract Structure
- [x] Create Contract struct with basic functionality
- [x] Implement state storage:
  - Tile storage using Map
  - Config item for contract configuration
- [x] Add constants:
  - GRID_SIZE = 16
  - PIXEL_SIZE = 32

### 1.3. Message Definitions
- [x] Create InstantiateMsg
- [x] Create ExecuteMsg:
  - Add SetPixels message
  - Add UpdateConfig message
- [x] Create QueryMsg:
  - Add Config query
  - Add Tile query

### 1.4. State Management
- [x] Implement Config struct:
  - Owner address
  - Base URI
  - Name and symbol
- [x] Implement Tile storage:
  - Tile data structure
  - Pixel data structure
- [x] Add state validation helpers

### 1.5. Entry Points
- [x] Implement instantiate:
  - Validate config
  - Store initial state
- [x] Implement execute router:
  - Handle config updates
  - Handle pixel updates
- [x] Implement query router:
  - Handle config queries
  - Handle tile queries

### 1.6. Error Handling
- [x] Define ContractError enum:
  - Standard errors
  - Custom errors for pixel updates
  - Validation errors
- [x] Implement error conversion traits

## 2. Contract Features (Priority: High)

### 2.1. SetPixels Implementation
- [x] Implement message validation:
  - Check pixel coordinates
  - Verify color format
- [x] Add pixel storage:
  - Store tile data
  - Query tile data
- [x] Add event emission:
  - Pixel update events with old and new colors
  - Update metadata tracking

## 3. Testing (Priority: High)
- [x] Unit tests for contract core:
  - Config management
  - Pixel updates
  - Queries
- [x] Integration tests:
  - Full update workflow
  - Error conditions
- [x] Test pixel validation
- [x] Test color format validation

## 4. Documentation (Priority: Medium)
- [x] API documentation:
  - Message types
  - State management
  - Event attributes
- [ ] Integration guide:
  - Contract deployment
  - Pixel updates
  - Event handling
- [ ] Usage examples:
  - Basic pixel updates
  - Batch updates
  - Event monitoring
- [ ] Deployment instructions:
  - Contract upload
  - Initialization
  - Configuration

## 5. Security (Priority: High)
- [x] Review access control:
  - Owner permissions
  - Update validation
- [x] Validate message size limits:
  - Batch update limits (MAX_PIXELS_PER_UPDATE)
  - Message size validation (MAX_MESSAGE_SIZE)
- [x] Review error handling:
  - Edge cases
  - Error messages
- [x] Test edge cases:
  - Boundary conditions
  - Invalid inputs
  - State transitions