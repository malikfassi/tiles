// Core domain modules
pub mod pricing;
pub mod quote;
pub mod tile;
pub mod validation;

// Re-export commonly used types
pub use pricing::PriceScaling;
pub use quote::quote;
pub use tile::Tile;
