use crate::common::helpers::mock::default_price_scaling;
use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;

#[test]
fn test_calculate_price() {
    let pricing = default_price_scaling();
    let current_time = 1000;

    // Test 1 hour duration
    let price = pricing.calculate_price(current_time + 3600, current_time);
    assert_eq!(price, pricing.hour_1_price);

    // Test 12 hour duration
    let price = pricing.calculate_price(current_time + 43200, current_time);
    assert_eq!(price, pricing.hour_12_price);

    // Test 24 hour duration
    let price = pricing.calculate_price(current_time + 86400, current_time);
    assert_eq!(price, pricing.hour_24_price);

    // Test beyond 24 hours
    let price = pricing.calculate_price(current_time + 90000, current_time);
    assert!(price > pricing.quadratic_base);
}

#[test]
fn test_calculate_total_price() {
    let pricing = default_price_scaling();
    let current_time = 1000;
    let expirations = vec![
        current_time + 3600,  // 1 hour
        current_time + 43200, // 12 hours
        current_time + 86400, // 24 hours
    ];

    let total = pricing.calculate_total_price(&expirations, current_time);
    let expected = pricing.hour_1_price + pricing.hour_12_price + pricing.hour_24_price;
    assert_eq!(total, expected);
}

#[test]
fn test_validate_price_scaling() {
    let valid = PriceScaling {
        hour_1_price: Uint128::new(100),
        hour_12_price: Uint128::new(200),
        hour_24_price: Uint128::new(300),
        quadratic_base: Uint128::new(400),
    };
    assert!(valid.validate().is_ok());

    let invalid = PriceScaling {
        hour_1_price: Uint128::zero(),
        hour_12_price: Uint128::new(200),
        hour_24_price: Uint128::new(300),
        quadratic_base: Uint128::new(400),
    };
    assert!(invalid.validate().is_err());
}
