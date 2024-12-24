use std::fmt::Debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::config::DecimalOps;

pub mod calculation;
pub mod validation;

pub use validation::{validate_price_scaling, validate_royalty_percent};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceScaling<D: Clone + Debug + PartialEq + DecimalOps> {
    pub hour_1_price: D,
    pub hour_12_price: D,
    pub hour_24_price: D,
    pub quadratic_base: D,
}

impl<D: Clone + Debug + PartialEq + DecimalOps + Default> Default for PriceScaling<D> {
    fn default() -> Self {
        Self {
            hour_1_price: D::default(),
            hour_12_price: D::default(),
            hour_24_price: D::default(),
            quadratic_base: D::default(),
        }
    }
} 