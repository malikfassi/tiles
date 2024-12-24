use std::fmt::Debug;
use cosmwasm_std::{Addr, Decimal};

/// Trait for decimal operations required by config
pub trait DecimalOps: Clone + Debug + PartialEq {
    fn one() -> Self;
    fn percent(value: u64) -> Self;
    fn is_zero(&self) -> bool;
    fn pow(&self, exp: u32) -> Self;
    fn gt(&self, other: &Self) -> bool;
    fn lt(&self, other: &Self) -> bool;
}

/// Trait for address operations required by config
pub trait AddressOps: Clone + Debug + PartialEq {
    fn as_str(&self) -> &str;
    fn to_string(&self) -> String;
}

impl AddressOps for Addr {
    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn to_string(&self) -> String {
        self.to_string()
    }
}

impl DecimalOps for Decimal {
    fn one() -> Self {
        Decimal::one()
    }

    fn percent(value: u64) -> Self {
        Decimal::percent(value)
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }

    fn pow(&self, exp: u32) -> Self {
        self.pow(exp)
    }

    fn gt(&self, other: &Self) -> bool {
        self > other
    }

    fn lt(&self, other: &Self) -> bool {
        self < other
    }
}
