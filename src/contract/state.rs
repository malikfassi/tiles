use crate::core::pricing::PriceScaling;
use cw_storage_plus::Item;

pub const PRICE_SCALING: Item<PriceScaling> = Item::new("price_scaling");
