use cw_storage_plus::Item;
use crate::core::config::Config;

pub const CONFIG: Item<Config> = Item::new("config");
