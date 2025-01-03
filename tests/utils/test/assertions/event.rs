use anyhow::Result;
use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::AppResponse;
use tiles::core::{pricing::PriceScaling, tile::metadata::PixelUpdate};

use crate::utils::state::{events::EventParser, tracker::StateTracker};

pub struct EventAssertions;

impl EventAssertions {
    pub fn assert_pixel_update(
        response: &AppResponse,
        token_id: u32,
        updates: &[&PixelUpdate],
        sender: &Addr,
    ) {
        let parsed_event =
            EventParser::parse_pixel_update(response).expect("Failed to parse pixel update event");

        assert_eq!(
            parsed_event.token_id,
            token_id.to_string(),
            "Token ID mismatch"
        );

        assert_eq!(
            parsed_event.new_pixels.len(),
            updates.len(),
            "Number of pixel updates mismatch"
        );

        for update in updates {
            let matching_pixel = parsed_event
                .new_pixels
                .iter()
                .find(|p| p.id == update.id)
                .unwrap_or_else(|| panic!("No matching pixel found for id {}", update.id));

            assert_eq!(matching_pixel.color, update.color, "Color mismatch");
            assert_eq!(
                matching_pixel.last_updated_by,
                sender.clone(),
                "Sender mismatch"
            );

            assert!(
                matching_pixel.expiration_timestamp > matching_pixel.last_updated_at,
                "Expiration timestamp must be after last updated time"
            );
            assert_eq!(
                matching_pixel.expiration_timestamp,
                matching_pixel.last_updated_at + update.expiration_duration,
                "Expiration timestamp must be last_updated_at + duration"
            );
        }
    }

    pub fn assert_mint_metadata(
        response: &AppResponse,
        token_id: u32,
        owner: &Addr,
        expected_hash: Option<&str>,
    ) {
        let parsed = EventParser::parse_mint_metadata(response)
            .expect("Failed to parse mint metadata event");

        assert_eq!(
            parsed.token_id.parse::<u32>().unwrap(),
            token_id,
            "Token ID mismatch"
        );
        assert_eq!(parsed.owner, owner.to_string(), "Owner mismatch");
        if let Some(expected) = expected_hash {
            assert_eq!(parsed.tile_hash, expected, "Hash mismatch");
        }
    }

    pub fn assert_price_scaling_update(response: &AppResponse, scaling: &PriceScaling) {
        let parsed = EventParser::parse_price_scaling_update(response)
            .expect("Failed to parse price scaling update event");

        assert_eq!(
            Uint128::from(parsed.hour_1_price),
            scaling.hour_1_price,
            "Hour 1 price mismatch"
        );
        assert_eq!(
            Uint128::from(parsed.hour_12_price),
            scaling.hour_12_price,
            "Hour 12 price mismatch"
        );
        assert_eq!(
            Uint128::from(parsed.hour_24_price),
            scaling.hour_24_price,
            "Hour 24 price mismatch"
        );
        assert_eq!(
            Uint128::from(parsed.quadratic_base),
            scaling.quadratic_base,
            "Quadratic base mismatch"
        );
    }

    pub fn assert_payment_distribution(
        response: &AppResponse,
        token_id: u32,
        sender: &Addr,
        state: &StateTracker,
        updates: &[&PixelUpdate],
    ) {
        let parsed = EventParser::parse_payment_distribution(response)
            .expect("Failed to parse payment distribution event");

        let price_scaling = state
            .get_price_scaling()
            .expect("Failed to get price scaling from state tracker");
        let total_price =
            price_scaling.calculate_total_price(updates.iter().map(|u| &u.expiration_duration));
        let (expected_royalty_amount, expected_owner_amount) =
            price_scaling.calculate_royalty_amounts(total_price);

        assert_eq!(parsed.token_id, token_id.to_string(), "Token ID mismatch");
        assert_eq!(parsed.sender, sender.to_string(), "Sender mismatch");
        assert_eq!(
            parsed.royalty_amount,
            expected_royalty_amount.u128(),
            "Royalty amount mismatch"
        );
        assert_eq!(
            parsed.owner_amount,
            expected_owner_amount.u128(),
            "Owner amount mismatch"
        );
    }

    pub fn assert_instantiate(response: &AppResponse) -> Result<()> {
        let event = EventParser::find_event(response, "instantiate")?;
        assert!(
            event
                .attributes
                .iter()
                .any(|attr| attr.key == "_contract_address"),
            "Instantiate event missing _contract_address attribute"
        );
        Ok(())
    }

    pub fn assert_instantiate_price_scaling(
        response: &AppResponse,
        expected: &PriceScaling,
    ) -> Result<()> {
        let event = EventParser::parse_instantiate_event(response)?;
        let parsed_scaling: PriceScaling = serde_json::from_str(&event.price_scaling)?;
        assert_eq!(parsed_scaling, *expected, "Price scaling mismatch");
        Ok(())
    }
}
