pub mod mock_app;
pub mod setup;

pub use mock_app::*;
pub use setup::TestSetup;

use cosmwasm_std::Addr;
use tiles::core::tile::metadata::PixelUpdate;

pub fn get_test_pixel_update(id: u32) -> PixelUpdate {
    PixelUpdate {
        id,
        color: "#FF0000".to_string(),
        expiration_duration: 3600, // 1 hour
    }
}
