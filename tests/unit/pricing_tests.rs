use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;
use tiles::defaults::constants::{ONE_HOUR, TWELVE_HOURS, TWENTY_FOUR_HOURS};

#[test]
fn test_calculate_price() {
    let pricing = PriceScaling::default();

    // Test 1 hour duration
    let price = pricing.calculate_price(ONE_HOUR);
    assert_eq!(price, pricing.hour_1_price);

    // Test 12 hour duration
    let price = pricing.calculate_price(TWELVE_HOURS);
    assert_eq!(price, pricing.hour_12_price);

    // Test 24 hour duration
    let price = pricing.calculate_price(TWENTY_FOUR_HOURS);
    assert_eq!(price, pricing.hour_24_price);

    // Test beyond 24 hours
    let extra_seconds = 1000;
    let price = pricing.calculate_price(TWENTY_FOUR_HOURS + extra_seconds);
    assert_eq!(
        price,
        pricing.quadratic_base + Uint128::from(extra_seconds * extra_seconds)
    );
}

#[test]
fn test_calculate_total_price() {
    let pricing = PriceScaling::default();
    let durations = [ONE_HOUR, TWELVE_HOURS, TWENTY_FOUR_HOURS];

    let total = pricing.calculate_total_price(durations.iter());
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

#[test]
fn test_default_price_scaling() {
    let default_pricing = PriceScaling::default();
    assert_eq!(default_pricing.hour_1_price, Uint128::from(100_000_000u128));
    assert_eq!(
        default_pricing.hour_12_price,
        Uint128::from(200_000_000u128)
    );
    assert_eq!(
        default_pricing.hour_24_price,
        Uint128::from(300_000_000u128)
    );
    assert_eq!(
        default_pricing.quadratic_base,
        Uint128::from(400_000_000u128)
    );
}
