use anyhow::Result;
use cosmwasm_std::Event;
use cw_multi_test::AppResponse;
use tiles::events::EventData;

pub struct EventParser {}

impl EventParser {
    pub fn find_events<'a>(response: &'a AppResponse, event_type: &str) -> Vec<&'a Event> {
        response
            .events
            .iter()
            .filter(|event| event.ty == event_type)
            .collect()
    }

    pub fn find_event<'a>(response: &'a AppResponse, event_type: &str) -> Result<&'a Event> {
        Self::find_events(response, event_type)
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No event found with type {}", event_type))
    }

    pub fn find_and_parse<T: EventData>(response: &AppResponse) -> Result<T> {
        let event = Self::find_event(response, T::event_type().as_wasm_str().as_str())?;
        T::try_from_event(event)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse event of type {}", event.ty))
    }

    pub fn find_and_parse_many<T: EventData>(response: &AppResponse) -> Result<Vec<T>> {
        let events = Self::find_events(response, T::event_type().as_wasm_str().as_str());
        events
            .into_iter()
            .map(|event| {
                T::try_from_event(event)
                    .ok_or_else(|| anyhow::anyhow!("Failed to parse event of type {}", event.ty))
            })
            .collect()
    }

    pub fn extract_token_id(response: &AppResponse) -> Result<u32> {
        // Find all wasm events
        let events = Self::find_events(response, "wasm");

        // Find the mint event from the sg721 contract
        let mint_event = events
            .iter()
            .find(|event| {
                event
                    .attributes
                    .iter()
                    .any(|attr| attr.key == "action" && attr.value == "mint")
            })
            .ok_or_else(|| anyhow::anyhow!("No mint event found"))?;

        mint_event
            .attributes
            .iter()
            .find(|attr| attr.key == "token_id")
            .and_then(|attr| attr.value.parse().ok())
            .ok_or_else(|| anyhow::anyhow!("Failed to extract token_id from mint event"))
    }
}
