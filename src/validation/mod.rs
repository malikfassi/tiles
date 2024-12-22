pub mod config;
pub mod pixel;

// Re-export commonly used validation functions
pub use pixel::{
    // Basic validations
    basic::{color, expiration, id},
    // Composite validations
    composite::{pixel_data, pixel_update, tile_metadata},
    // Message validations
    message::size as validate_message_size,
};

pub use config::validate_config_update;
