use cosmwasm_std::{Decimal, Uint128};
use tiles::core::pricing::{PriceScaling, calculation::calculate_pixel_price};

#[test]
fn test_price_calculation() {
    let price_scaling = PriceScaling {
        hour_1_price: Decimal::percent(10),
        hour_12_price: Decimal::percent(20),
        hour_24_price: Decimal::percent(30),
        quadratic_base: Decimal::percent(40),
    };

    // Test hourly pricing tiers
    assert_eq!(calculate_pixel_price(&price_scaling, 1).unwrap(), Uint128::new(10));
    assert_eq!(calculate_pixel_price(&price_scaling, 12).unwrap(), Uint128::new(20));
    assert_eq!(calculate_pixel_price(&price_scaling, 24).unwrap(), Uint128::new(30));

    // Test quadratic scaling
    let price = calculate_pixel_price(&price_scaling, 48).unwrap();
    let base = Uint128::new(40);
    assert!(price > base, "price: {}, base: {}", price, base);
    
    let price_36h = calculate_pixel_price(&price_scaling, 36).unwrap();
    assert!(price > price_36h, "48h price: {}, 36h price: {}", price, price_36h);
} 