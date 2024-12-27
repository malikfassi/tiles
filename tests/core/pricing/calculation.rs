use cosmwasm_std::Uint128;
use tiles::core::pricing::PriceScaling;
use tiles::defaults::constants::{ONE_HOUR, TWELVE_HOURS, TWENTY_FOUR_HOURS};

#[test]
fn price_is_fixed_at_30_minutes() {
    let scaling = PriceScaling::default();
    let price = scaling.calculate_price(ONE_HOUR / 2);
    assert_eq!(price, scaling.hour_1_price);
}

#[test]
fn price_is_fixed_at_one_hour() {
    let scaling = PriceScaling::default();
    let price = scaling.calculate_price(ONE_HOUR);
    assert_eq!(price, scaling.hour_1_price);
}

#[test]
fn price_at_6_hours_is_interpolated() {
    let scaling = PriceScaling::default();
    let six_hours = ONE_HOUR * 6;
    let price = scaling.calculate_price(six_hours);

    // Calculate expected price with integer division
    let progress = Uint128::from((six_hours - ONE_HOUR) as u128)
        .checked_mul(Uint128::from(1_000_000u128))
        .unwrap()
        .checked_div(Uint128::from((TWELVE_HOURS - ONE_HOUR) as u128))
        .unwrap();
    let price_diff = scaling.hour_12_price.saturating_sub(scaling.hour_1_price);
    let expected = scaling.hour_1_price
        + price_diff
            .checked_mul(progress)
            .unwrap()
            .checked_div(Uint128::from(1_000_000u128))
            .unwrap();

    assert_eq!(
        price, expected,
        "Price at 6 hours should match linear interpolation"
    );
}

#[test]
fn price_at_12_hours_matches_hour_12_price() {
    let scaling = PriceScaling::default();
    let price = scaling.calculate_price(TWELVE_HOURS);
    assert_eq!(price, scaling.hour_12_price);
}

#[test]
fn price_at_18_hours_is_interpolated() {
    let scaling = PriceScaling::default();
    let eighteen_hours = ONE_HOUR * 18;
    let price = scaling.calculate_price(eighteen_hours);

    // Calculate expected price with integer division
    let progress = Uint128::from((eighteen_hours - TWELVE_HOURS) as u128)
        .checked_mul(Uint128::from(1_000_000u128))
        .unwrap()
        .checked_div(Uint128::from((TWENTY_FOUR_HOURS - TWELVE_HOURS) as u128))
        .unwrap();
    let price_diff = scaling.hour_24_price.saturating_sub(scaling.hour_12_price);
    let expected = scaling.hour_12_price
        + price_diff
            .checked_mul(progress)
            .unwrap()
            .checked_div(Uint128::from(1_000_000u128))
            .unwrap();

    assert_eq!(
        price, expected,
        "Price at 18 hours should match linear interpolation"
    );
}

#[test]
fn price_at_24_hours_matches_hour_24_price() {
    let scaling = PriceScaling::default();
    let price = scaling.calculate_price(TWENTY_FOUR_HOURS);
    assert_eq!(price, scaling.hour_24_price);
}

#[test]
fn price_beyond_24_hours_is_quadratic() {
    let scaling = PriceScaling::default();
    let duration = TWENTY_FOUR_HOURS + 100;
    let price = scaling.calculate_price(duration);

    let expected = scaling.quadratic_base + Uint128::from(100u64 * 100u64);
    assert_eq!(price, expected);
}

#[test]
fn total_price_sums_individual_prices() {
    let scaling = PriceScaling::default();
    let durations = [ONE_HOUR / 2, ONE_HOUR * 6];

    let total = scaling.calculate_total_price(durations.iter());
    let sum = scaling.calculate_price(ONE_HOUR / 2) + scaling.calculate_price(ONE_HOUR * 6);

    assert_eq!(total, sum);
}
