use cosmwasm_std::Event;

mod instantiate_config;
mod metadata_update;
mod mint_metadata;
mod payment_distribution;
mod pixel_update;
mod price_scaling;

pub use instantiate_config::InstantiateConfigEventData;
pub use metadata_update::MetadataUpdateEventData;
pub use mint_metadata::MintMetadataEventData;
pub use payment_distribution::PaymentDistributionEventData;
pub use pixel_update::PixelUpdateEventData;
pub use price_scaling::PriceScalingUpdateEventData;

#[derive(Debug, Clone, Copy)]
pub enum EventType {
    PixelUpdateEvent,
    MetadataUpdateEvent,
    PaymentDistributionEvent,
    PriceScalingUpdateEvent,
    InstantiateConfigEvent,
    MintMetadataEvent,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::PixelUpdateEvent => "pixel_update",
            EventType::MetadataUpdateEvent => "metadata_update",
            EventType::PaymentDistributionEvent => "payment_distribution",
            EventType::PriceScalingUpdateEvent => "price_scaling_update",
            EventType::InstantiateConfigEvent => "instantiate_config",
            EventType::MintMetadataEvent => "mint_metadata",
        }
    }

    pub fn as_wasm_str(&self) -> String {
        format!("wasm-{}", self.as_str())
    }
}

pub trait EventData {
    fn event_type() -> EventType;
    fn into_event(self) -> Event;
    fn try_from_event(event: &Event) -> Option<Self>
    where
        Self: Sized;
}
