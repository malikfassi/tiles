use cosmwasm_std::{Addr, Attribute};

/// Event types for standardization
#[derive(Debug, Clone, Copy)]
pub enum EventType {
    SetPixelColor,
    UpdateConfig,
    PaymentProcessed,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::SetPixelColor => "set_pixel_color",
            EventType::UpdateConfig => "update_config",
            EventType::PaymentProcessed => "payment_processed",
        }
    }
}

/// Generate standard attributes for pixel update events
pub fn pixel_update_attributes(
    tiles_updated: u32,
    pixels_updated: u32,
    updater: &Addr,
) -> Vec<Attribute> {
    vec![
        ("action", EventType::SetPixelColor.as_str()),
        ("tiles_updated", &tiles_updated.to_string()),
        ("pixels_updated", &pixels_updated.to_string()),
        ("updater", updater.as_str()),
    ]
    .into_iter()
    .map(|(k, v)| Attribute::new(k, v))
    .collect()
}

/// Generate standard attributes for config update events
pub fn config_update_attributes(updated_fields: &[&str]) -> Vec<Attribute> {
    vec![
        ("action", EventType::UpdateConfig.as_str()),
        ("updated_fields", &updated_fields.join(",")),
    ]
    .into_iter()
    .map(|(k, v)| Attribute::new(k, v))
    .collect()
}

/// Generate standard attributes for payment events
pub fn payment_attributes(
    total_amount: u128,
    denom: &str,
    royalty_amount: Option<u128>,
) -> Vec<Attribute> {
    let mut attrs = vec![
        ("action", EventType::PaymentProcessed.as_str()),
        ("total_amount", &total_amount.to_string()),
        ("denom", denom),
    ];

    if let Some(royalty) = royalty_amount {
        attrs.push(("royalty_amount", &royalty.to_string()));
    }

    attrs.into_iter()
        .map(|(k, v)| Attribute::new(k, v))
        .collect()
} 