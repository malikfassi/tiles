use cosmwasm_std::{Attribute, Event};
use serde::{Deserialize, Serialize};

use crate::events::{EventData, EventType};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MintMetadataEventData {
    pub token_id: String,
    pub owner: String,
    pub token_uri: String,
    pub tile_hash: String,
    pub time: String,
}

impl EventData for MintMetadataEventData {
    fn event_type() -> EventType {
        EventType::MintMetadataEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attributes(vec![
                Attribute::new("token_id", self.token_id),
                Attribute::new("owner", self.owner),
                Attribute::new("token_uri", self.token_uri),
                Attribute::new("tile_hash", self.tile_hash),
                Attribute::new("time", self.time),
            ])
    }

    fn try_from_event(event: &Event) -> Option<Self> {
        if event.ty != Self::event_type().as_wasm_str() {
            return None;
        }

        let mut token_id = None;
        let mut owner = None;
        let mut token_uri = None;
        let mut tile_hash = None;
        let mut time = None;

        for attr in &event.attributes {
            match attr.key.as_str() {
                "token_id" => token_id = Some(attr.value.clone()),
                "owner" => owner = Some(attr.value.clone()),
                "token_uri" => token_uri = Some(attr.value.clone()),
                "tile_hash" => tile_hash = Some(attr.value.clone()),
                "time" => time = Some(attr.value.clone()),
                _ => {}
            }
        }

        Some(Self {
            token_id: token_id?,
            owner: owner?,
            token_uri: token_uri?,
            tile_hash: tile_hash?,
            time: time?,
        })
    }
} 