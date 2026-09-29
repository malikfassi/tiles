use crate::events::{EventData, EventType};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Event;

/// Emitted on a successful migration, so an indexer can follow schema changes.
#[cw_serde]
pub struct MigrationEventData {
    pub from_version: String,
    pub to_version: String,
}

impl EventData for MigrationEventData {
    fn event_type() -> EventType {
        EventType::MigrationEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str())
            .add_attribute("from_version", self.from_version)
            .add_attribute("to_version", self.to_version)
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
            from_version: get_attr("from_version")?,
            to_version: get_attr("to_version")?,
        })
    }
}
