use cosmwasm_std::{Addr, Event};
use serde::{Deserialize, Serialize};

use super::{EventData, EventType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PixelUpdateEventData {
    pub token_id: String,
    pub pixel_id: u32,
    pub color: String,
    pub expiration_duration: u64,
    pub expiration_timestamp: u64,
    pub last_updated_by: Addr,
    pub last_updated_at: u64,
}

impl EventData for PixelUpdateEventData {
    fn event_type() -> EventType {
        EventType::PixelUpdateEvent
    }
    
    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attribute("token_id", self.token_id)
            .add_attribute("pixel_id", self.pixel_id.to_string())
            .add_attribute("color", self.color)
            .add_attribute("expiration_duration", self.expiration_duration.to_string())
            .add_attribute("expiration_timestamp", self.expiration_timestamp.to_string())
            .add_attribute("last_updated_by", self.last_updated_by.to_string())
            .add_attribute("last_updated_at", self.last_updated_at.to_string())
    }
    
    fn try_from_event(event: &Event) -> Option<Self> {
        if event.ty != Self::event_type().as_wasm_str() {
            return None;
        }
        
        let get_attr = |key: &str| {
            event.attributes
                .iter()
                .find(|a| a.key == key)
                .map(|a| a.value.clone())
        };
        
        Some(Self {
            token_id: get_attr("token_id")?,
            pixel_id: get_attr("pixel_id")?.parse().ok()?,
            color: get_attr("color")?,
            expiration_duration: get_attr("expiration_duration")?.parse().ok()?,
            expiration_timestamp: get_attr("expiration_timestamp")?.parse().ok()?,
            last_updated_by: Addr::unchecked(get_attr("last_updated_by")?),
            last_updated_at: get_attr("last_updated_at")?.parse().ok()?,
        })
    }
} 