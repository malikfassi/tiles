use anyhow::Result;
use cosmwasm_std::{Addr, Event, Uint128};
use cw_multi_test::AppResponse;

use tiles::{
    core::{pricing::PriceScaling, tile::metadata::PixelUpdate},
    events::{
        EventData, InstantiatePriceScalingEventData, MintMetadataEventData,
        PaymentDistributionEventData, PixelUpdateEventData, PriceScalingUpdateEventData,
    },
};

use crate::utils::{events::EventParser, state::StateTracker};

pub struct ResponseAssertions {}

impl ResponseAssertions {
    pub fn assert_pixel_update(
        response: &AppResponse,
        token_id: u32,
        updates: &[&PixelUpdate],
        sender: &Addr,
    ) {
        let parsed_event = EventParser::find_and_parse::<PixelUpdateEventData>(response)
            .expect("Failed to parse pixel update event");

        // Verify token ID matches
        assert_eq!(
            parsed_event.token_id,
            token_id.to_string(),
            "Token ID mismatch"
        );

        // Verify we have the right number of pixel updates
        assert_eq!(
            parsed_event.new_pixels.len(),
            updates.len(),
            "Number of pixel updates mismatch"
        );

        // Verify each update has a matching pixel with correct data
        for update in updates {
            let matching_pixel = parsed_event
                .new_pixels
                .iter()
                .find(|p| p.id == update.id)
                .unwrap_or_else(|| panic!("No matching pixel found for id {}", update.id));

            // Verify all fields match exactly what was requested
            assert_eq!(matching_pixel.color, update.color, "Color mismatch");
            assert_eq!(
                matching_pixel.last_updated_by,
                sender.clone(),
                "Sender mismatch"
            );

            // Verify timestamp is set correctly
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
        let events = EventParser::find_and_parse_many::<MintMetadataEventData>(response)
            .expect("Failed to parse mint metadata events");
        assert!(!events.is_empty(), "No mint metadata event found");

        let parsed = events.first().unwrap();

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

    pub fn assert_instantiate_price_scaling(response: &AppResponse, expected_minter: &str) {
        let parsed_event =
            EventParser::find_and_parse::<InstantiatePriceScalingEventData>(response)
                .expect("Failed to parse instantiate price scaling event");

        assert!(
            !parsed_event.collection_info.is_empty(),
            "Collection info should not be empty"
        );
        assert_eq!(
            parsed_event.minter, expected_minter,
            "Minter address mismatch"
        );
        assert!(
            !parsed_event.price_scaling.is_empty(),
            "Price scaling should not be empty"
        );
        assert!(!parsed_event.time.is_empty(), "Time should not be empty");
    }

    pub fn assert_payment_distribution(
        response: &AppResponse,
        token_id: u32,
        sender: &Addr,
        state: &StateTracker,
        updates: &[&PixelUpdate],
    ) {
        let parsed_event = EventParser::find_and_parse::<PaymentDistributionEventData>(response)
            .expect("Failed to parse payment distribution event");

        // Calculate expected amounts using state tracker's price scaling
        let price_scaling = state
            .get_price_scaling()
            .expect("Failed to get price scaling from state tracker");
        let total_price =
            price_scaling.calculate_total_price(updates.iter().map(|u| &u.expiration_duration));
        let (expected_royalty_amount, expected_owner_amount) =
            price_scaling.calculate_royalty_amounts(total_price);

        assert_eq!(
            parsed_event.token_id,
            token_id.to_string(),
            "Token ID mismatch"
        );
        assert_eq!(parsed_event.sender, sender.to_string(), "Sender mismatch");
        assert_eq!(
            parsed_event.royalty_amount,
            expected_royalty_amount.u128(),
            "Royalty amount mismatch"
        );
        assert_eq!(
            parsed_event.owner_amount,
            expected_owner_amount.u128(),
            "Owner amount mismatch"
        );
    }

    pub fn assert_price_scaling_update(response: &AppResponse, scaling: &PriceScaling) {
        let parsed_event = EventParser::find_and_parse::<PriceScalingUpdateEventData>(response)
            .expect("Failed to parse price scaling update event");

        assert_eq!(
            Uint128::from(parsed_event.hour_1_price),
            scaling.hour_1_price,
            "Hour 1 price mismatch"
        );
        assert_eq!(
            Uint128::from(parsed_event.hour_12_price),
            scaling.hour_12_price,
            "Hour 12 price mismatch"
        );
        assert_eq!(
            Uint128::from(parsed_event.hour_24_price),
            scaling.hour_24_price,
            "Hour 24 price mismatch"
        );
        assert_eq!(
            Uint128::from(parsed_event.quadratic_base),
            scaling.quadratic_base,
            "Quadratic base mismatch"
        );
    }
}
