use cosmwasm_std::Addr;
use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;

pub const NATIVE_DENOM: &str = "ustars";

pub fn default_price_scaling() -> PriceScaling {
    PriceScaling {
        hour_1_price: Uint128::new(100),
        hour_12_price: Uint128::new(200),
        hour_24_price: Uint128::new(300),
        quadratic_base: Uint128::new(400),
    }
}

pub fn mock_creator() -> Addr {
    Addr::unchecked("creator")
}

pub fn mock_user() -> Addr {
    Addr::unchecked("user")
} 