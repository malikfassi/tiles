use cosmwasm_std::{Addr, Event};
use serde::{Deserialize, Serialize};

use super::{EventData, EventType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentDistributionEventData {
    pub token_id: String,
    pub sender: Addr,
    pub collection_amount: u128,
    pub platform_amount: u128,
    pub owner_amount: u128,
    pub total: u128,
}

impl EventData for PaymentDistributionEventData {
    fn event_type() -> EventType {
        EventType::PaymentDistributionEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attribute("token_id", self.token_id)
            .add_attribute("sender", self.sender.to_string())
            .add_attribute("collection_amount", self.collection_amount.to_string())
            .add_attribute("platform_amount", self.platform_amount.to_string())
            .add_attribute("owner_amount", self.owner_amount.to_string())
            .add_attribute("total", self.total.to_string())
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
            token_id: get_attr("token_id")?,
            sender: Addr::unchecked(get_attr("sender")?),
            collection_amount: get_attr("collection_amount")?.parse().ok()?,
            platform_amount: get_attr("platform_amount")?.parse().ok()?,
            owner_amount: get_attr("owner_amount")?.parse().ok()?,
            total: get_attr("total")?.parse().ok()?,
        })
    }
}
