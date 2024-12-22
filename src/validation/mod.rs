pub mod pixel;
pub mod config;

// Re-export commonly used validation functions
pub use pixel::{
    // Basic validations
    basic::{color, expiration, id},
    // Message validations
    message::size as validate_message_size,
    // Composite validations
    composite::{pixel_data, pixel_update, tile_metadata},
};

pub use config::validate_config_update;