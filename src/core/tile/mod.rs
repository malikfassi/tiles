use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod metadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Tile {
    pub tile_hash: String,
}