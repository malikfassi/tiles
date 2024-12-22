use anyhow::Result;
use cosmwasm_std::{Addr, Coin, Decimal, Event, Uint128};
use cw_multi_test::{App, Executor};
use cw721::{OwnerOfResponse, TokensResponse};
use sg721_base::msg::QueryMsg as Sg721QueryMsg;

use tiles::msg::{ConfigResponse, QueryMsg, SetPixelColorMsg, TileStateResponse, UpdateConfigMsg};
use tiles::types::{PriceScaling, PixelData, PixelUpdate, TileUpdate, TileUpdates, TileMetadata};
use tiles::utils::price::calculate_price;
use tiles::defaults::constants::{DEFAULT_PIXEL_COLOR, PIXELS_PER_TILE};

use crate::common::constants::NATIVE_DENOM;

pub struct TilesContract {
    pub address: Addr,
}

impl TilesContract {
    pub fn new(address: Addr) -> Self {
        Self { address }
    }

    pub fn query_config(&self, app: &App) -> Result<ConfigResponse> {
        Ok(app.wrap()
            .query_wasm_smart(self.address.clone(), &QueryMsg::Config {})?)
    }

    pub fn query_tile_state(&self, app: &App, token_id: &str) -> Result<TileStateResponse> {
        Ok(app.wrap().query_wasm_smart(
            self.address.clone(),
            &QueryMsg::TileState {
                token_id: token_id.to_string(),
            },
        )?)
    }

    pub fn query_owner_of(&self, app: &App, token_id: &str) -> Result<String> {
        let res: OwnerOfResponse = app.wrap().query_wasm_smart(
            self.address.clone(),
            &QueryMsg::Sg721(Sg721QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: None,
            }),
        )?;
        Ok(res.owner)
    }

    pub fn query_tokens(
        &self,
        app: &App,
        owner: &str,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<String>> {
        let res: TokensResponse = app.wrap().query_wasm_smart(
            self.address.clone(),
            &QueryMsg::Sg721(Sg721QueryMsg::Tokens {
                owner: owner.to_string(),
                start_after,
                limit,
            }),
        )?;
        Ok(res.tokens)
    }

    pub fn update_config(
        &self,
        app: &mut App,
        sender: &Addr,
        tiles_royalty_payment_address: Option<String>,
        tiles_royalties: Option<Decimal>,
        price_scaling: Option<PriceScaling>,
    ) -> Result<()> {
        app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &UpdateConfigMsg {
                tiles_royalty_payment_address,
                tiles_royalties,
                price_scaling,
            },
            &[],
        )?;
        Ok(())
    }

    pub fn set_pixel_color(
        &self,
        app: &mut App,
        sender: &Addr,
        msg: SetPixelColorMsg,
    ) -> Result<()> {
        app.execute_contract(sender.clone(), self.address.clone(), &msg, &[])?;
        Ok(())
    }

    /// Create default pixel update for testing
    pub fn default_pixel_update(id: u32) -> PixelUpdate {
        PixelUpdate {
            id,
            color: "#FF0000".to_string(), // Red
            expiration: 100, // 100 seconds
        }
    }

    /// Create default tile metadata for testing
    pub fn default_tile_metadata(
        tile_id: &str,
        owner: &Addr,
        timestamp: u64,
    ) -> TileMetadata {
        let mut pixels = Vec::with_capacity(PIXELS_PER_TILE as usize);
        for i in 0..PIXELS_PER_TILE {
            pixels.push(PixelData {
                id: i,
                color: DEFAULT_PIXEL_COLOR.to_string(),
                expiration: timestamp,
                last_updated_by: owner.clone(),
                last_updated_at: timestamp,
            });
        }
        TileMetadata {
            tile_id: tile_id.to_string(),
            pixels,
        }
    }

    /// Create a pixel update for testing
    pub fn create_pixel_update(
        &self,
        id: u32,
        color: &str,
        expiration: u64,
    ) -> PixelUpdate {
        PixelUpdate {
            id,
            color: color.to_string(),
            expiration,
        }
    }

    /// Create a tile update for testing
    pub fn create_tile_update(
        &self,
        token_id: &str,
        current_pixels: Vec<PixelData>,
        pixel_updates: Vec<PixelUpdate>,
    ) -> TileUpdate {
        TileUpdate {
            tile_id: token_id.to_string(),
            current_metadata: TileMetadata {
                tile_id: token_id.to_string(),
                pixels: current_pixels,
            },
            updates: TileUpdates {
                pixels: pixel_updates,
            },
        }
    }

    /// Create payment coins for testing
    pub fn create_payment(amount: u128) -> Vec<Coin> {
        vec![Coin::new(amount, NATIVE_DENOM)]
    }

    /// Query account balance
    pub fn query_balance(&self, app: &App, address: &str) -> Result<Uint128> {
        Ok(app.wrap().query_balance(address, NATIVE_DENOM)?.amount)
    }

    /// Assert account balance
    pub fn assert_balance(
        &self,
        app: &App,
        address: &str,
        expected_amount: Uint128,
    ) -> Result<()> {
        let balance = self.query_balance(app, address)?;
        assert_eq!(
            balance,
            expected_amount,
            "Balance mismatch for {}",
            address
        );
        Ok(())
    }

    /// Execute pixel update with payment
    pub fn execute_pixel_update(
        &self,
        app: &mut App,
        sender: &Addr,
        updates: Vec<TileUpdate>,
        payment: Option<Vec<Coin>>,
    ) -> Result<()> {
        let msg = SetPixelColorMsg { updates };
        app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &msg,
            &payment.unwrap_or_default(),
        )?;
        Ok(())
    }

    /// Calculate expected fees for a pixel update
    pub fn calculate_fees(
        &self,
        app: &App,
        duration: u64,
    ) -> Result<(Uint128, Uint128)> {
        let config = self.query_config(app)?;
        let total_fee = calculate_price(&config.price_scaling, duration);
        let royalty_fee = total_fee * config.tiles_royalties;
        Ok((total_fee, royalty_fee))
    }
}
