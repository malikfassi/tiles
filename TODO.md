# CosmWasm Standards Alignment TODO

## 1. Hash Generation
- [ ] Replace string formatting-based hashing with canonical serialization
- [ ] Use `serde` for deterministic hashing of metadata
- [ ] Consider using `cosmwasm_std::Binary` for hash representation instead of hex string

## 2. Validation Pattern
- [ ] Move validation logic from data structures to execute handlers
- [ ] Split validation into:
  - [ ] Message validation (in execute handler)
  - [ ] State validation (in execute handler)
  - [ ] Business rules validation (in execute handler)
- [ ] Remove `validate_integrity` and `validate_for_tile` from `PixelUpdate`

## 3. Error Handling
- [ ] Replace dynamic error messages with specific error variants
- [ ] Add specific error types for:
  - [ ] `InvalidColorFormat`
  - [ ] `InvalidPixelId`
  - [ ] `InvalidExpirationTooShort`
  - [ ] `InvalidExpirationTooLong`
- [ ] Remove string formatting from error construction

## 4. State Management
- [ ] Replace Vec<PixelData> with fixed-size array [PixelData; 100]
- [ ] Consider more granular storage patterns:
  - [ ] Store pixels individually
  - [ ] Use composite keys for efficient queries
- [ ] Optimize state updates for gas efficiency

## 5. Testing
- [ ] Add unit tests for state serialization
- [ ] Add integration tests for state migrations
- [ ] Add test coverage for all error cases
- [ ] Add gas usage benchmarks

## 6. Documentation
- [ ] Add rustdoc comments following CosmWasm patterns
- [ ] Document state management approach
- [ ] Document validation flow
- [ ] Add examples for common operations

## 7. Optimizations
- [ ] Memory Optimizations:
  - [ ] Use `&str` instead of `String` for color field (since it's always 7 chars)
  - [ ] Use compact timestamps (u32 instead of u64 if possible)
  - [ ] Consider using bit flags for pixel states

- [ ] Gas Optimizations:
  - [ ] Batch state updates to reduce storage writes
  - [ ] Cache frequently accessed data in memory
  - [ ] Minimize cloning of Addr and other heap-allocated types
  - [ ] Use `update` instead of `load`+`save` for state modifications

- [ ] Storage Optimizations:
  - [ ] Use prefix iteration instead of loading full state
  - [ ] Consider using nested maps for better query performance
  - [ ] Implement partial state updates for pixels
  - [ ] Use composite keys to optimize common queries

- [ ] Message Size Optimizations:
  - [ ] Use compact message formats (e.g., shorter field names)
  - [ ] Implement batch operations to reduce number of transactions
  - [ ] Consider using bit-packed structures for pixel updates

- [ ] Query Optimizations:
  - [ ] Add pagination for large result sets
  - [ ] Implement efficient filtering at contract level
  - [ ] Cache common query results
  - [ ] Add composite indexes for frequent query patterns

## 8. File-Specific Improvements

### src/contract/
- [ ] mod.rs:
  - [ ] Remove module inception (contract module inside contract)
  - [ ] Use proper re-exports

- [ ] error.rs:
  - [ ] Add specific error variants instead of generic messages
  - [ ] Remove string formatting from errors
  - [ ] Add error codes for better client handling

- [ ] execute.rs:
  - [ ] Move validation logic from data structures here
  - [ ] Implement proper state management patterns
  - [ ] Add proper response attributes

- [ ] msg.rs:
  - [ ] Use more compact field names
  - [ ] Add proper validation attributes
  - [ ] Consider using custom serialization

### src/core/
- [ ] pricing.rs:
  - [ ] Use fixed-point arithmetic for better precision
  - [ ] Cache common calculations
  - [ ] Add bounds checking

- [ ] tile/metadata.rs:
  - [ ] Use fixed-size array instead of Vec
  - [ ] Optimize hash calculation
  - [ ] Use &str for color field
  - [ ] Use compact timestamps

### src/events/
- [ ] mod.rs:
  - [ ] Use proper event attribute keys
  - [ ] Implement proper event filtering
  - [ ] Add event versioning

- [ ] pixel_update.rs:
  - [ ] Optimize event attribute storage
  - [ ] Add proper indexing attributes
  - [ ] Use compact event data format

### src/defaults/
- [ ] constants.rs:
  - [ ] Add proper documentation
  - [ ] Consider using const generics
  - [ ] Add validation ranges

## 9. Type Optimizations

### Pixel ID
- [ ] Change pixel ID from `u32` to `u8`:
  - [ ] In `PixelData` struct (id field)
  - [ ] In `PixelUpdate` struct (id field)
  - [ ] Update validation bounds check to use `u8::MAX`
  - [ ] Update error messages to use u8
  - [ ] Update tests to use u8 values
  - Rationale:
    - PIXELS_PER_TILE = 100, so u8 (max 255) is more than sufficient
    - Saves 3 bytes per pixel in storage
    - More efficient for serialization/deserialization
    - Better represents the domain constraints

### Related Changes
- [ ] Update constants:
  - [ ] Consider making PIXELS_PER_TILE a u8 instead of u32
  - [ ] Add type assertion test to ensure PIXELS_PER_TILE fits in u8
