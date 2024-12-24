use std::fmt::Debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::traits::{AddressOps, DecimalOps};

/// Configuration for a tile contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config<A: Clone + Debug + PartialEq + AddressOps, D: Clone + Debug + PartialEq + DecimalOps> {
    /// Address that receives royalty payments
    pub royalty_payment_address: A,
    /// Percentage of each sale that goes to royalty_payment_address (0-1)
    pub royalty_percent: D,
    /// Pricing configuration for different time periods
    pub price_scaling: PriceScaling<D>,
}

/// Price scaling configuration for different time periods
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceScaling<D: Clone + Debug + PartialEq + DecimalOps> {
    /// Price for 1 hour of pixel ownership
    pub hour_1_price: D,
    /// Price for 12 hours of pixel ownership
    pub hour_12_price: D,
    /// Price for 24 hours of pixel ownership
    pub hour_24_price: D,
    /// Base price for quadratic pricing formula
    pub quadratic_base: D,
}

impl<D: Clone + Debug + PartialEq + DecimalOps> PriceScaling<D> {
    pub fn new(hour_1_price: D, hour_12_price: D, hour_24_price: D, quadratic_base: D) -> Self {
        Self {
            hour_1_price,
            hour_12_price,
            hour_24_price,
            quadratic_base,
        }
    }

    pub fn or(self, other: Self) -> Self {
        Self {
            hour_1_price: self.hour_1_price,
            hour_12_price: self.hour_12_price,
            hour_24_price: self.hour_24_price,
            quadratic_base: self.quadratic_base,
        }
    }
}
