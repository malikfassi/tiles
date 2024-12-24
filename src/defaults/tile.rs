use crate::core::tile::metadata::TileMetadata;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_TILE_METADATA: TileMetadata = TileMetadata {
        pixels: vec![],
    };
}
