use cosmwasm_std::{Addr, Event};
use cw_multi_test::AppResponse;
use tiles::{
    core::{pricing::PriceScaling, tile::metadata::PixelUpdate},
    events::{
        EventData, InstantiateConfigEventData, MintMetadataEventData, PaymentDistributionEventData,
        PixelUpdateEventData, PriceScalingUpdateEventData,
    },
};

pub struct EventAssertions {}

impl EventAssertions {
    pub fn find_events<'a>(response: &'a AppResponse, event_type: &str) -> Vec<&'a Event> {
        println!("Looking for event type: {}", event_type);
        println!("Available events:");
        for event in &response.events {
            println!("Event type: {}", event.ty);
            println!("Attributes: {:?}", event.attributes);
        }
        response
            .events
            .iter()
            .filter(|event| event.ty == event_type)
            .collect()
    }

    fn find_event<'a>(response: &'a AppResponse, event_type: &str) -> &'a Event {
        Self::find_events(response, event_type)
            .first()
            .unwrap_or_else(|| {
                let available_events = response
                    .events
                    .iter()
                    .map(|e| format!("{} with attributes: {:?}", e.ty, e.attributes))
                    .collect::<Vec<_>>()
                    .join("\n");
                panic!(
                    "Expected event of type '{}' but found:\n{}",
                    event_type, available_events
                )
            })
    }

    /// Extracts the token_id from a mint response
    pub fn extract_token_id(response: &AppResponse) -> u32 {
        let events = Self::find_events(
            response,
            MintMetadataEventData::event_type().as_wasm_str().as_str(),
        );
        assert!(!events.is_empty(), "No mint metadata event found");

        let event = events.first().unwrap();
        println!("Found mint event: {:?}", event);
        let parsed = MintMetadataEventData::try_from_event(event)
            .expect("Failed to parse mint metadata event");
        parsed
            .token_id
            .parse::<u32>()
            .expect("Failed to parse token_id as u32")
    }

    pub fn assert_mint_metadata(
        response: &AppResponse,
        token_id: u32,
        owner: &Addr,
        expected_hash: Option<&str>,
    ) {
        let events = Self::find_events(
            response,
            MintMetadataEventData::event_type().as_wasm_str().as_str(),
        );
        assert!(!events.is_empty(), "No mint metadata event found");

        let event = events.first().unwrap();
        let parsed = MintMetadataEventData::try_from_event(event)
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

    pub fn assert_instantiate_config(response: &AppResponse, expected_minter: &str) {
        let event = Self::find_event(
            response,
            InstantiateConfigEventData::event_type()
                .as_wasm_str()
                .as_str(),
        );
        let parsed_event = InstantiateConfigEventData::try_from_event(event).unwrap_or_else(|| {
            panic!(
                "Failed to parse instantiate config event. Event: {:?}",
                event
            )
        });

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

    pub fn assert_pixel_update(
        response: &AppResponse,
        token_id: u32,
        updates: &[PixelUpdate],
        sender: &Addr,
    ) {
        let events = Self::find_events(
            response,
            PixelUpdateEventData::event_type().as_wasm_str().as_str(),
        );

        // We should have exactly one event per update
        assert_eq!(
            events.len(),
            updates.len(),
            "Expected exactly {} pixel update events, found {}",
            updates.len(),
            events.len()
        );

        // Verify each update has a matching event with correct data
        for update in updates {
            let matching_event = events
                .iter()
                .find(|event| {
                    if let Some(parsed) = PixelUpdateEventData::try_from_event(event) {
                        parsed.token_id == token_id.to_string() && parsed.pixel_id == update.id
                    } else {
                        false
                    }
                })
                .unwrap_or_else(|| panic!("No matching event found for pixel {}", update.id));

            let parsed_event = PixelUpdateEventData::try_from_event(matching_event).unwrap();

            // Verify all fields match exactly what was requested
            assert_eq!(
                parsed_event.token_id,
                token_id.to_string(),
                "Token ID mismatch"
            );
            assert_eq!(parsed_event.pixel_id, update.id, "Pixel ID mismatch");
            assert_eq!(parsed_event.color, update.color, "Color mismatch");
            assert_eq!(
                parsed_event.expiration_duration, update.expiration_duration,
                "Duration mismatch"
            );
            assert_eq!(
                parsed_event.last_updated_by,
                sender.to_string(),
                "Sender mismatch"
            );

            // Verify timestamp is set correctly
            assert!(
                parsed_event.expiration_timestamp > parsed_event.last_updated_at,
                "Expiration timestamp must be after last updated time"
            );
            assert_eq!(
                parsed_event.expiration_timestamp,
                parsed_event.last_updated_at + update.expiration_duration,
                "Expiration timestamp must be last_updated_at + duration"
            );
        }
    }

    pub fn assert_payment_distribution(
        response: &AppResponse,
        token_id: u32,
        sender: &Addr,
        royalty_amount: u128,
        owner_amount: u128,
    ) {
        let event = Self::find_event(
            response,
            PaymentDistributionEventData::event_type()
                .as_wasm_str()
                .as_str(),
        );
        let parsed_event =
            PaymentDistributionEventData::try_from_event(event).unwrap_or_else(|| {
                panic!(
                    "Failed to parse payment distribution event. Event: {:?}",
                    event
                )
            });

        assert_eq!(
            parsed_event.token_id,
            token_id.to_string(),
            "Token ID mismatch"
        );
        assert_eq!(parsed_event.sender, sender.to_string(), "Sender mismatch");
        assert_eq!(
            parsed_event.royalty_amount.to_string(),
            royalty_amount.to_string(),
            "Royalty amount mismatch"
        );
        assert_eq!(
            parsed_event.owner_amount.to_string(),
            owner_amount.to_string(),
            "Owner amount mismatch"
        );
    }

    pub fn assert_price_scaling_update(response: &AppResponse, scaling: &PriceScaling) {
        let event = Self::find_event(
            response,
            PriceScalingUpdateEventData::event_type()
                .as_wasm_str()
                .as_str(),
        );
        let parsed_event =
            PriceScalingUpdateEventData::try_from_event(event).unwrap_or_else(|| {
                panic!(
                    "Failed to parse price scaling update event. Event: {:?}",
                    event
                )
            });

        assert_eq!(
            parsed_event.hour_1_price.to_string(),
            scaling.hour_1_price.u128().to_string(),
            "Hour 1 price mismatch"
        );
        assert_eq!(
            parsed_event.hour_12_price.to_string(),
            scaling.hour_12_price.u128().to_string(),
            "Hour 12 price mismatch"
        );
        assert_eq!(
            parsed_event.hour_24_price.to_string(),
            scaling.hour_24_price.u128().to_string(),
            "Hour 24 price mismatch"
        );
        assert_eq!(
            parsed_event.quadratic_base.to_string(),
            scaling.quadratic_base.u128().to_string(),
            "Quadratic base mismatch"
        );
    }
}
