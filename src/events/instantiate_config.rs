use cosmwasm_std::{Attribute, Event};
use serde::{Deserialize, Serialize};

use crate::events::{EventData, EventType};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateConfigEventData {
    pub collection_info: String,
    pub minter: String,
    pub price_scaling: String,
    pub time: String,
}

impl EventData for InstantiateConfigEventData {
    fn event_type() -> EventType {
        EventType::InstantiateConfigEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attributes(vec![
                Attribute::new("collection_info", self.collection_info),
                Attribute::new("minter", self.minter),
                Attribute::new("price_scaling", self.price_scaling),
                Attribute::new("time", self.time),
            ])
    }

    fn try_from_event(event: &Event) -> Option<Self> {
        if event.ty != Self::event_type().as_str() {
            return None;
        }

        let mut collection_info = None;
        let mut minter = None;
        let mut price_scaling = None;
        let mut time = None;

        for attr in &event.attributes {
            match attr.key.as_str() {
                "collection_info" => collection_info = Some(attr.value.clone()),
                "minter" => minter = Some(attr.value.clone()),
                "price_scaling" => price_scaling = Some(attr.value.clone()),
                "time" => time = Some(attr.value.clone()),
                _ => {}
            }
        }

        Some(Self {
            collection_info: collection_info?,
            minter: minter?,
            price_scaling: price_scaling?,
            time: time?,
        })
    }
} 