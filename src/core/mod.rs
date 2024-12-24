// Core domain modules
pub mod config;
pub mod pricing;
pub mod tile;

// Re-export commonly used types
pub use config::{AddressOps, Config, DecimalOps};
pub use pricing::PriceScaling;
pub use tile::Tile;