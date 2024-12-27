use cosmwasm_std::Event;
use serde::{Deserialize, Serialize};

use super::{EventData, EventType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataUpdateEventData {
    pub token_id: String,
    pub resulting_hash: String,
}

impl EventData for MetadataUpdateEventData {
    fn event_type() -> EventType {
        EventType::MetadataUpdateEvent
    }
    
    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attribute("token_id", self.token_id)
            .add_attribute("resulting_hash", self.resulting_hash)
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
            resulting_hash: get_attr("resulting_hash")?,
        })
    }
} 