use anyhow::Result;
use cosmwasm_std::Uint128;
use sg_std::NATIVE_DENOM;
use tiles::{
    core::{pricing::PriceScaling, tile::metadata::PixelUpdate},
    defaults::constants::DEFAULT_ROYALTY_SHARE,
};

use crate::utils::{ResponseAssertions, StateAssertions, TestSetup};

#[test]
fn payment_is_distributed_correctly() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();
    let token_id = setup.mint_token(&buyer.address)?;

    let pixel_id = 1;
    let color = "#FF0000";
    let duration_hours = 1;

    let update = PixelUpdate {
        id: pixel_id,
        color: color.to_string(),
        expiration_duration: duration_hours * 3600,
    };

    let price_scaling = setup.state.get_price_scaling()?;
    let expected_payment = price_scaling.calculate_price(duration_hours * 3600);
    println!("\nPrice scaling: {:?}", price_scaling);
    println!("Expected payment: {}", expected_payment);

    let initial_creator_balance = setup
        .app
        .get_balance(&setup.users.tile_contract_creator().address, NATIVE_DENOM)?;
    let initial_owner_balance = setup.app.get_balance(&buyer.address, NATIVE_DENOM)?;
    println!("Initial creator balance: {}", initial_creator_balance);
    println!("Initial owner balance: {}", initial_owner_balance);

    let response = setup.update_pixel(&buyer.address, token_id, vec![update.clone()])?;

    // Calculate royalty amounts using price scaling
    let (royalty_payment, owner_payment) =
        price_scaling.calculate_royalty_amounts(expected_payment);
    println!("Royalty payment: {}", royalty_payment);
    println!("Owner payment: {}", owner_payment);

    // Verify creator received royalty payment
    let final_creator_balance = setup
        .app
        .get_balance(&setup.users.tile_contract_creator().address, NATIVE_DENOM)?;
    println!("Final creator balance: {}", final_creator_balance);
    StateAssertions::assert_balance(
        &setup.app,
        &setup.users.tile_contract_creator().address,
        initial_creator_balance + royalty_payment.u128(),
    );

    // Verify owner received remaining payment
    let final_owner_balance = setup.app.get_balance(&buyer.address, NATIVE_DENOM)?;
    println!("Final owner balance: {}", final_owner_balance);
    StateAssertions::assert_balance(
        &setup.app,
        &buyer.address,
        initial_owner_balance - expected_payment.u128() + owner_payment.u128(),
    );

    // Verify events
    ResponseAssertions::assert_pixel_update(&response, token_id, &[&update], &buyer.address);
    ResponseAssertions::assert_payment_distribution(
        &response,
        token_id,
        &buyer.address,
        &setup.state,
        &[&update],
    );

    Ok(())
}
