use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_multi_test::{AppResponse, Executor};
use sg_multi_test::StargazeApp;
use tiles::msg::Extension;

use crate::common::NATIVE_DENOM;

pub struct TilesContract<'a> {
    pub app: &'a mut StargazeApp,
    pub contract_addr: String,
    pub owner: String,
}

impl<'a> TilesContract<'a> {
    pub fn new(app: &'a mut StargazeApp, owner: &str) -> Self {
        Self {
            app,
            contract_addr: String::new(),
            owner: owner.to_string(),
        }
    }

    pub fn mint(&mut self, token_id: &str) -> anyhow::Result<AppResponse> {
        let msg = tiles::msg::ExecuteMsg::Base(sg721::ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: self.owner.clone(),
            token_uri: None,
            extension: Extension::default(),
        });

        self.app.execute_contract(
            Addr::unchecked(&self.owner),
            Addr::unchecked(&self.contract_addr),
            &msg,
            &[],
        )
    }

    pub fn set_pixel_color(
        &mut self,
        token_id: &str,
        pixels: Vec<tiles::msg::PixelUpdate>,
    ) -> anyhow::Result<AppResponse> {
        let msg = tiles::msg::ExecuteMsg::SetPixelColor(tiles::msg::SetPixelColorMsg {
            updates: vec![tiles::msg::TileUpdate {
                tile_id: token_id.to_string(),
                current_metadata: tiles::state::TileMetadata {
                    tile_id: token_id.to_string(),
                    pixels: vec![],
                },
                updates: tiles::msg::TileUpdates { pixels },
            }],
            max_message_size: 128 * 1024,
        });

        self.app.execute_contract(
            Addr::unchecked(&self.owner),
            Addr::unchecked(&self.contract_addr),
            &msg,
            &[Coin::new(100_000_000, NATIVE_DENOM)],
        )
    }

    pub fn update_config(
        &mut self,
        dev_address: Option<String>,
        dev_fee_percent: Option<Decimal>,
        base_price: Option<Uint128>,
        price_scaling: Option<tiles::state::PriceScaling>,
    ) -> anyhow::Result<AppResponse> {
        let msg = tiles::msg::ExecuteMsg::UpdateConfig {
            dev_address,
            dev_fee_percent,
            base_price,
            price_scaling,
        };

        self.app.execute_contract(
            Addr::unchecked(&self.owner),
            Addr::unchecked(&self.contract_addr),
            &msg,
            &[],
        )
    }
} 