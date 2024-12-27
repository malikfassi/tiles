use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::AppResponse;
use std::collections::HashMap;
use tiles::contract::error::ContractError;
use tiles::core::tile::metadata::PixelUpdate;

use super::launchpad::Launchpad;

pub struct TestOrchestrator {
    pub ctx: Launchpad,
    pub state: TestState,
}

#[derive(Default)]
pub struct TestState {
    pub minted_tokens: HashMap<Addr, Vec<u32>>, // owner -> token_ids
    pub pixel_updates: HashMap<u32, Vec<PixelUpdate>>, // token_id -> updates
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

    pub fn assert_pixel_color(&self, _token_id: u32, _pixel_id: u32, _expected_color: &str) {
        // TODO: Implement once we have a way to query pixel colors
        unimplemented!("Pixel color assertion not yet implemented");
    }

    // Error assertion helpers
    pub fn assert_error_invalid_config(&self, result: Result<AppResponse>, expected_msg: &str) {
        match result {
            Err(err) => {
                let contract_err: ContractError = err.downcast().unwrap();
                match contract_err {
                    ContractError::InvalidConfig(msg) => assert_eq!(msg, expected_msg),
                    _ => panic!("Expected InvalidConfig error, got: {:?}", contract_err),
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
                    ContractError::HashMismatch {} => (),
                    _ => panic!("Expected HashMismatch error, got: {:?}", contract_err),
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
                    ContractError::Unauthorized {} => (),
                    _ => panic!("Expected Unauthorized error, got: {:?}", contract_err),
                }
            }
            Ok(_) => panic!("Expected error, got success"),
        }
    }

    // Event assertion helpers
    pub fn assert_pixel_update_event(&self, response: &AppResponse, token_id: &str, sender: &Addr) {
        let event = response
            .events
            .iter()
            .find(|e| {
                e.ty == "wasm"
                    && e.attributes
                        .iter()
                        .any(|a| a.key == "action" && a.value == "set_pixel_color")
            })
            .expect("Expected set_pixel_color event");

        let token_id_attr = event
            .attributes
            .iter()
            .find(|a| a.key == "token_id")
            .expect("Expected token_id attribute");
        assert_eq!(token_id_attr.value, token_id);

        let sender_attr = event
            .attributes
            .iter()
            .find(|a| a.key == "sender")
            .expect("Expected sender attribute");
        assert_eq!(sender_attr.value, sender.to_string());
    }

    pub fn assert_token_hash(&self, token_id: u32, expected_hash: &str) -> Result<()> {
        let hash = self.ctx.tiles.query_token_hash(&self.ctx.app, token_id)?;
        assert_eq!(hash, expected_hash, "Token hash mismatch");
        Ok(())
    }
}
