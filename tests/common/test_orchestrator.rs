use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;

use cosmwasm_std::Event;
use std::collections::HashMap;
use tiles::contract::error::ContractError;
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};
use tiles::events::{
    EventData, PaymentDistributionEventData, PixelUpdateEventData, PriceScalingUpdateEventData,
};

use super::launchpad::Launchpad;

pub struct TestOrchestrator {
    pub ctx: Launchpad,
    pub state: TestState,
}

#[derive(Default)]
pub struct TestState {
    pub minted_tokens: HashMap<Addr, Vec<u32>>, // owner -> token_ids
    pub pixel_updates: HashMap<u32, Vec<PixelUpdate>>, // token_id -> updates
    pub token_metadata: HashMap<u32, TileMetadata>, // token_id -> current metadata
}

impl Default for TestOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl TestOrchestrator {
    pub fn new() -> Self {
        Self {
            ctx: Launchpad::new(),
            state: TestState::default(),
        }
    }

    // State setup helpers
    pub fn mint_token(&mut self, owner: &Addr) -> Result<u32> {
        let token_id = self.ctx.minter.mint(&mut self.ctx.app, owner)?;
        self.state
            .minted_tokens
            .entry(owner.clone())
            .or_default()
            .push(token_id);
        // Track default metadata after minting
        self.state
            .token_metadata
            .insert(token_id, TileMetadata::default());
        Ok(token_id)
    }

    pub fn mint_tokens(&mut self, owner: &Addr, count: u32) -> Result<Vec<u32>> {
        let mut token_ids = Vec::new();
        for _ in 0..count {
            token_ids.push(self.mint_token(owner)?);
        }
        Ok(token_ids)
    }

    // Common test states
    pub fn setup_single_token(&mut self) -> Result<(Addr, u32)> {
        let owner = self.ctx.users.get_buyer().address.clone();
        let token_id = self.mint_token(&owner)?;
        Ok((owner, token_id))
    }

    pub fn setup_multiple_tokens(&mut self, count: u32) -> Result<(Addr, Vec<u32>)> {
        let owner = self.ctx.users.get_buyer().address.clone();
        let token_ids = self.mint_tokens(&owner, count)?;
        Ok((owner, token_ids))
    }

    // Assertions
    pub fn assert_token_owner(&self, token_id: u32, expected_owner: &Addr) {
        self.ctx
            .tiles
            .assert_token_owner(&self.ctx.app, token_id, expected_owner);
    }

    // Error assertion helpers
    pub fn assert_error_invalid_config(&self, result: Result<AppResponse>, expected_msg: &str) {
        match result {
            Err(err) => {
                let contract_err: ContractError = err.downcast().unwrap();
                match contract_err {
                    ContractError::InvalidPixelId { id } => {
                        assert_eq!(expected_msg, format!("Invalid pixel id: {}", id))
                    },
                    ContractError::DuplicatePixelId { id } => {
                        assert_eq!(expected_msg, format!("Duplicate pixel id: {}", id))
                    },
                    ContractError::InvalidPixelUpdate { reason } => {
                        assert_eq!(expected_msg, reason)
                    },
                    _ => panic!("Expected InvalidPixelId, DuplicatePixelId, or InvalidPixelUpdate error, got: {:?}", contract_err),
                }
            }
            Ok(_) => panic!("Expected error, got success"),
        }
    }

    pub fn assert_error_hash_mismatch(&self, result: Result<AppResponse>) {
        match result {
            Err(err) => {
                let contract_err: ContractError = err.downcast().unwrap();
                match contract_err {
                    ContractError::MetadataHashMismatch {} => (),
                    _ => panic!(
                        "Expected MetadataHashMismatch error, got: {:?}",
                        contract_err
                    ),
                }
            }
            Ok(_) => panic!("Expected error, got success"),
        }
    }

    pub fn assert_error_unauthorized(&self, result: Result<AppResponse>) {
        match result {
            Err(err) => {
                let contract_err: ContractError = err.downcast().unwrap();
                match contract_err {
                    ContractError::Unauthorized { sender: _ } => (),
                    ContractError::Base(sg721_base::ContractError::Base(
                        cw721_base::ContractError::Ownership(cw721_base::OwnershipError::NotOwner),
                    )) => (),
                    _ => panic!("Expected Unauthorized error, got: {:?}", contract_err),
                }
            }
            Ok(_) => panic!("Expected error, got success"),
        }
    }

    // Pixel update event assertions
    pub fn assert_pixel_update_event(
        &self,
        response: &AppResponse,
        token_id: &str,
        update: &PixelUpdate,
        sender: &Addr,
    ) {
        let events = self.find_events(
            response,
            PixelUpdateEventData::event_type().as_wasm_str().as_str(),
        );
        let parsed_event = events
            .iter()
            .find_map(|event| {
                let parsed = PixelUpdateEventData::try_from_event(event)?;
                if parsed.pixel_id == update.id {
                    Some(parsed)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                panic!(
                    "Failed to find pixel update event for pixel ID {}. Events: {:?}",
                    update.id, events
                )
            });

        // Assert predictable values
        assert_eq!(parsed_event.token_id, token_id, "Token ID mismatch");
        assert_eq!(parsed_event.pixel_id, update.id, "Pixel ID mismatch");
        assert_eq!(parsed_event.color, update.color, "Color mismatch");
        assert_eq!(
            parsed_event.expiration_duration, update.expiration_duration,
            "Expiration duration mismatch"
        );
        assert_eq!(
            parsed_event.last_updated_by,
            sender.clone(),
            "Last updated by mismatch"
        );

        // Verify timestamps exist and are valid
        assert!(
            parsed_event.last_updated_at > 0,
            "last_updated_at should be set"
        );
        assert!(
            parsed_event.expiration_timestamp > parsed_event.last_updated_at,
            "expiration_timestamp should be after last_updated_at"
        );
        assert_eq!(
            parsed_event.expiration_timestamp - parsed_event.last_updated_at,
            update.expiration_duration,
            "expiration_timestamp should be last_updated_at + expiration_duration"
        );
    }

    // Payment event assertions
    pub fn assert_payment_distribution_event(
        &self,
        response: &AppResponse,
        token_id: &str,
        sender: &Addr,
        royalty_amount: u128,
        owner_amount: u128,
    ) {
        let event = self.find_event(
            response,
            PaymentDistributionEventData::event_type()
                .as_wasm_str()
                .as_str(),
        );
        let parsed_event =
            PaymentDistributionEventData::try_from_event(event).unwrap_or_else(|| {
                panic!(
                    "Failed to parse payment distribution event. Event: {:?}",
                    event
                )
            });

        assert_eq!(parsed_event.token_id, token_id, "Token ID mismatch");
        assert_eq!(parsed_event.sender, sender.clone(), "Sender mismatch");
        assert_eq!(
            parsed_event.royalty_amount, royalty_amount,
            "Royalty amount mismatch"
        );
        assert_eq!(
            parsed_event.owner_amount, owner_amount,
            "Owner amount mismatch"
        );
    }

    // Price scaling event assertions
    pub fn assert_price_scaling_event(&self, response: &AppResponse, expected: PriceScaling) {
        let event = self.find_event(
            response,
            PriceScalingUpdateEventData::event_type()
                .as_wasm_str()
                .as_str(),
        );
        let parsed_event =
            PriceScalingUpdateEventData::try_from_event(event).unwrap_or_else(|| {
                panic!(
                    "Failed to parse price scaling update event. Event: {:?}",
                    event
                )
            });

        assert_eq!(
            parsed_event.hour_1_price,
            expected.hour_1_price.u128(),
            "Hour 1 price mismatch"
        );
        assert_eq!(
            parsed_event.hour_12_price,
            expected.hour_12_price.u128(),
            "Hour 12 price mismatch"
        );
        assert_eq!(
            parsed_event.hour_24_price,
            expected.hour_24_price.u128(),
            "Hour 24 price mismatch"
        );
        assert_eq!(
            parsed_event.quadratic_base,
            expected.quadratic_base.u128(),
            "Quadratic base mismatch"
        );
    }

    pub fn assert_token_hash(&self, token_id: u32, expected_hash: &str) -> Result<()> {
        let hash = self.ctx.tiles.query_token_hash(&self.ctx.app, token_id)?;
        assert_eq!(hash, expected_hash, "Token hash mismatch");
        Ok(())
    }

    // Fund assertion helpers
    pub fn assert_funds_received(&self, address: &Addr, amount: u128, denom: &str) {
        let balance = self
            .ctx
            .app
            .inner()
            .wrap()
            .query_balance(address, denom)
            .unwrap();
        assert_eq!(
            balance.amount.u128(),
            amount,
            "Balance mismatch for {}: expected {}, got {}",
            address,
            amount,
            balance.amount.u128()
        );
    }

    pub fn track_pixel_update(
        &mut self,
        token_id: u32,
        update: PixelUpdate,
        response: &AppResponse,
    ) {
        // Track the update
        self.state
            .pixel_updates
            .entry(token_id)
            .or_default()
            .push(update);

        // Get or create metadata
        let metadata = self.state.token_metadata.entry(token_id).or_default();

        // Extract metadata from events
        if let Some(event_data) = response
            .events
            .iter()
            .find_map(PixelUpdateEventData::try_from_event)
        {
            let pixel_id = event_data.pixel_id as usize;
            if pixel_id < metadata.pixels.len() {
                metadata.pixels[pixel_id].id = event_data.pixel_id;
                metadata.pixels[pixel_id].color = event_data.color;
                metadata.pixels[pixel_id].expiration_timestamp = event_data.expiration_timestamp;
                metadata.pixels[pixel_id].last_updated_by = event_data.last_updated_by;
                metadata.pixels[pixel_id].last_updated_at = event_data.last_updated_at;
            }
        }
    }

    pub fn get_current_metadata(&self, token_id: u32) -> TileMetadata {
        self.state
            .token_metadata
            .get(&token_id)
            .cloned()
            .unwrap_or_default()
    }

    // update pixels with just updates
    pub fn update_pixels(
        &mut self,
        token_id: u32,
        updates: Vec<PixelUpdate>,
        operator: &Addr,
    ) -> Result<AppResponse> {
        let current_metadata = self.get_current_metadata(token_id);
        let response = self.ctx.tiles.update_pixel(
            &mut self.ctx.app,
            operator,
            token_id,
            updates.clone(),
            current_metadata,
        )?;

        // Track each update
        for update in updates {
            self.track_pixel_update(token_id, update, &response);
        }
        Ok(response)
    }

    fn find_events<'a>(&self, response: &'a AppResponse, event_type: &str) -> Vec<&'a Event> {
        response
            .events
            .iter()
            .filter(|event| event.ty.replace("wasm-wasm-", "wasm-") == event_type)
            .collect()
    }

    fn find_event<'a>(&self, response: &'a AppResponse, event_type: &str) -> &'a Event {
        self.find_events(response, event_type)
            .first()
            .unwrap_or_else(|| {
                let available_events = response
                    .events
                    .iter()
                    .map(|e| format!("{} with attributes: {:?}", e.ty, e.attributes))
                    .collect::<Vec<_>>()
                    .join("\n");
                panic!(
                    "Expected event of type '{}' but found:\n{}",
                    event_type, available_events
                )
            })
    }
}
