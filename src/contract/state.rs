use crate::core::Config;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const TILE_CONFIG: Item<Config> = Item::new("tile_config");
pub const SG721_ADDRESS: Item<Addr> = Item::new("sg721_address");
