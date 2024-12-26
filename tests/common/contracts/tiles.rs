use crate::common::test_module::TilesApp as App;
use anyhow::Result;
use cosmwasm_std::{to_json_binary, Addr, Coin, Uint128};
use cw_multi_test::{AppResponse, Executor};
use tiles::contract::msg::{ExecuteMsg, TileExecuteMsg};
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};
use tiles::defaults::constants::{PIXEL_MIN_EXPIRATION, MINT_PRICE};
use sg_std::NATIVE_DENOM;
use vending_minter::msg::ExecuteMsg as MinterExecuteMsg;

use crate::common::helpers::setup::TestSetup;

pub struct TilesContract {
    pub address: Option<Addr>,
}

impl TilesContract {
    pub fn new(address: Addr) -> Self {
        Self {
            address: Some(address),
        }
    }

    pub fn mint_through_minter(
        &self,
        app: &mut App,
        owner: &Addr,
        minter: &Addr,
    ) -> Result<AppResponse> {
        let mint_msg = MinterExecuteMsg::Mint {};
        println!(
            "DEBUG: Sending vending minter message: {}",
            String::from_utf8_lossy(&to_json_binary(&mint_msg).unwrap())
        );

        app.execute_contract(
            owner.clone(),
            minter.clone(),
            &mint_msg,
            &[Coin::new(MINT_PRICE, NATIVE_DENOM)],
        )
        .map_err(Into::into)
    }

    pub fn update_pixel(
        &self,
        app: &mut App,
        owner: &Addr,
        token_id: u32,
        color: String,
    ) -> Result<AppResponse> {
        // Get current block time
        let current_time = app.block_info().time.seconds();

        // Create a pixel update with minimum expiration
        let update = PixelUpdate {
            id: 0, // Update first pixel
            color,
            expiration_duration: PIXEL_MIN_EXPIRATION,
        };

        app.execute_contract(
            owner.clone(),
            self.address.as_ref().unwrap().clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata: TileMetadata::default(),
                    updates: vec![update],
                },
            },
            &[Coin::new(MINT_PRICE, NATIVE_DENOM)], // Send same amount as mint price for now
        )
        .map_err(Into::into)
    }
}
