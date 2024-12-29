use cosmwasm_std::{Attribute, Event};
use serde::{Deserialize, Serialize};

use super::{EventData, EventType};
use crate::core::tile::metadata::PixelData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PixelUpdateEventData {
    pub token_id: String,
    pub new_pixels: Vec<PixelData>,
    pub tile_hash: String,
}

impl EventData for PixelUpdateEventData {
    fn event_type() -> EventType {
        EventType::PixelUpdateEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str()).add_attributes(vec![
            Attribute::new("token_id", self.token_id),
            Attribute::new("tile_hash", self.tile_hash),
            Attribute::new(
                "pixels",
                serde_json::to_string(&self.new_pixels).unwrap_or_default(),
            ),
        ])
    }

    fn try_from_event(event: &Event) -> Option<Self> {
        if event.ty != Self::event_type().as_wasm_str() {
            return None;
        }

        let get_attr = |key: &str| {
            event
                .attributes
                .iter()
                .find(|a| a.key == key)
                .map(|a| a.value.clone())
        };

        let token_id = get_attr("token_id")?;
        let tile_hash = get_attr("tile_hash")?;
        let new_pixels: Vec<PixelData> = serde_json::from_str(&get_attr("pixels")?).ok()?;

        Some(Self {
            token_id,
            new_pixels,
            tile_hash,
        })
    }
}
