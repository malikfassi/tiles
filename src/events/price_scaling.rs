use cosmwasm_std::Event;
use serde::{Deserialize, Serialize};

use super::{EventData, EventType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceScalingUpdateEventData {
    pub hour_1_price: u128,
    pub hour_12_price: u128,
    pub hour_24_price: u128,
    pub quadratic_base: u128,
}

impl EventData for PriceScalingUpdateEventData {
    fn event_type() -> EventType {
        EventType::PriceScalingUpdateEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attribute("hour_1_price", self.hour_1_price.to_string())
            .add_attribute("hour_12_price", self.hour_12_price.to_string())
            .add_attribute("hour_24_price", self.hour_24_price.to_string())
            .add_attribute("quadratic_base", self.quadratic_base.to_string())
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

        Some(Self {
            hour_1_price: get_attr("hour_1_price")?.parse().ok()?,
            hour_12_price: get_attr("hour_12_price")?.parse().ok()?,
            hour_24_price: get_attr("hour_24_price")?.parse().ok()?,
            quadratic_base: get_attr("quadratic_base")?.parse().ok()?,
        })
    }
}
