use cosmwasm_std::Decimal;

// Royalty configuration
pub const DEFAULT_ROYALTY_PERCENT: Decimal = Decimal::raw(50000000000000000); // 5%

// Price scaling configuration
pub const HOUR_1_PRICE: Decimal = Decimal::raw(1_000_000_000_000_000_000); // 1 STARS
pub const HOUR_12_PRICE: Decimal = Decimal::raw(2_000_000_000_000_000_000); // 2 STARS
pub const HOUR_24_PRICE: Decimal = Decimal::raw(3_000_000_000_000_000_000); // 3 STARS
pub const QUADRATIC_BASE: Decimal = Decimal::raw(4_000_000_000_000_000_000); // 4 STARS

// Network configuration
pub const NATIVE_DENOM: &str = "ustars";
 