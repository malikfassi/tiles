use cw_storage_plus::Item;

use crate::types::Config;

// Contract configuration
pub const CONFIG: Item<Config> = Item::new("config");
