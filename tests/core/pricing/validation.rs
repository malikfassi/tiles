use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;

#[test]
fn test_valid_price_scaling() {
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::new(1_000_000),
        hour_12_price: Uint128::new(10_000_000),
        hour_24_price: Uint128::new(20_000_000),
        quadratic_base: Uint128::new(1_000_000),
    };

    assert!(price_scaling.validate().is_ok());
}

#[test]
fn test_invalid_price_scaling() {
    // Test invalid price scaling (hour_12_price less than hour_1_price)
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::new(10_000_000),
        hour_12_price: Uint128::new(1_000_000),
        hour_24_price: Uint128::new(20_000_000),
        quadratic_base: Uint128::new(1_000_000),
    };

    assert!(price_scaling.validate().is_err());

    // Test invalid price scaling (hour_24_price less than hour_12_price)
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::new(1_000_000),
        hour_12_price: Uint128::new(20_000_000),
        hour_24_price: Uint128::new(10_000_000),
        quadratic_base: Uint128::new(1_000_000),
    };

    assert!(price_scaling.validate().is_err());
}

#[test]
fn test_zero_prices() {
    // Test zero hour_1_price
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::zero(),
        hour_12_price: Uint128::new(10_000_000),
        hour_24_price: Uint128::new(20_000_000),
        quadratic_base: Uint128::new(1_000_000),
    };
    assert!(price_scaling.validate().is_err());

    // Test zero hour_12_price
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::new(1_000_000),
        hour_12_price: Uint128::zero(),
        hour_24_price: Uint128::new(20_000_000),
        quadratic_base: Uint128::new(1_000_000),
    };
    assert!(price_scaling.validate().is_err());

    // Test zero hour_24_price
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::new(1_000_000),
        hour_12_price: Uint128::new(10_000_000),
        hour_24_price: Uint128::zero(),
        quadratic_base: Uint128::new(1_000_000),
    };
    assert!(price_scaling.validate().is_err());

    // Test zero quadratic_base
    let price_scaling = PriceScaling {
        hour_1_price: Uint128::new(1_000_000),
        hour_12_price: Uint128::new(10_000_000),
        hour_24_price: Uint128::new(20_000_000),
        quadratic_base: Uint128::zero(),
    };
    assert!(price_scaling.validate().is_err());
}
