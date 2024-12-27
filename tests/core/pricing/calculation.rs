use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;
use tiles::defaults::constants::{
    DEFAULT_PRICE_12_HOURS, DEFAULT_PRICE_1_HOUR, DEFAULT_PRICE_24_HOURS,
    DEFAULT_PRICE_QUADRATIC_BASE,
};

#[test]
fn test_price_scaling() {
    let scaling = PriceScaling::default();

    // Check default values
    assert_eq!(scaling.hour_1_price.u128(), DEFAULT_PRICE_1_HOUR);
    assert_eq!(scaling.hour_12_price.u128(), DEFAULT_PRICE_12_HOURS);
    assert_eq!(scaling.hour_24_price.u128(), DEFAULT_PRICE_24_HOURS);
    assert_eq!(scaling.quadratic_base.u128(), DEFAULT_PRICE_QUADRATIC_BASE);

    // Test price calculation for different durations
    let one_hour_price = scaling.calculate_price(3600); // 1 hour
    assert_eq!(one_hour_price.u128(), scaling.hour_1_price.u128());

    let twelve_hour_price = scaling.calculate_price(43200); // 12 hours
    assert_eq!(twelve_hour_price.u128(), scaling.hour_12_price.u128());

    let twenty_four_hour_price = scaling.calculate_price(86400); // 24 hours
    assert_eq!(twenty_four_hour_price.u128(), scaling.hour_24_price.u128());

    // Test price calculation for intermediate durations
    let six_hour_price = scaling.calculate_price(21600); // 6 hours
    assert!(six_hour_price.u128() > scaling.hour_1_price.u128());
    assert!(six_hour_price.u128() < scaling.hour_12_price.u128());

    let eighteen_hour_price = scaling.calculate_price(64800); // 18 hours
    assert!(eighteen_hour_price.u128() > scaling.hour_12_price.u128());
    assert!(eighteen_hour_price.u128() < scaling.hour_24_price.u128());
}
