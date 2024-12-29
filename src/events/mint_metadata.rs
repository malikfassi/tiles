use cosmwasm_std::{Addr, Attribute, Event};
use serde::{Deserialize, Serialize};

use crate::{
    core::tile::metadata::PixelData,
    events::{EventData, EventType},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MintMetadataEventData {
    pub token_id: String,
    pub owner: Addr,
    pub new_pixels: Vec<PixelData>,
    pub tile_hash: String,
}

impl EventData for MintMetadataEventData {
    fn event_type() -> EventType {
        EventType::MintMetadataEvent
    }

    fn into_event(self) -> Event {
        Event::new(Self::event_type().as_str()).add_attributes(vec![
            Attribute::new("token_id", self.token_id),
            Attribute::new("owner", self.owner.to_string()),
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
        let owner = Addr::unchecked(get_attr("owner")?);
        let tile_hash = get_attr("tile_hash")?;
        let new_pixels: Vec<PixelData> = serde_json::from_str(&get_attr("pixels")?).ok()?;

        Some(Self {
            token_id,
            owner,
            new_pixels,
            tile_hash,
        })
    }
}
