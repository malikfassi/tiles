pub mod mock_app;
pub mod setup;

pub use mock_app::*;
pub use setup::TestSetup;

use cosmwasm_std::Addr;
use tiles::core::tile::metadata::PixelUpdate;

pub fn mock_pixel_update(id: u32, color: &str, _sender: &Addr) -> PixelUpdate {
    PixelUpdate {
        id,
        color: color.to_string(),
        expiration: 3600, // 1 hour
    }
}
