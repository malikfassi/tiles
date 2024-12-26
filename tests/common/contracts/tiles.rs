use crate::common::test_module::TilesApp as App;
use anyhow::Result;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{AppResponse, Executor};
use sg_std::NATIVE_DENOM;
use tiles::contract::msg::{ExecuteMsg, TileExecuteMsg};
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::metadata::{PixelUpdate, TileMetadata};
use tiles::defaults::constants::{MINT_PRICE, PIXEL_MIN_EXPIRATION};
use vending_minter::msg::ExecuteMsg as MinterExecuteMsg;

pub struct TilesContract {
    pub contract_addr: Addr,
}

impl TilesContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    // Contract execution helpers
    pub fn mint_through_minter(
        &self,
        app: &mut App,
        owner: &Addr,
        minter: &Addr,
    ) -> Result<AppResponse, anyhow::Error> {
        app.execute_contract(
            owner.clone(),
            minter.clone(),
            &MinterExecuteMsg::Mint {},
            &[Coin::new(MINT_PRICE, NATIVE_DENOM)],
        )
    }

    pub fn update_pixel(
        &self,
        app: &mut App,
        owner: &Addr,
        token_id: u32,
        color: String,
    ) -> Result<AppResponse, anyhow::Error> {
        let price_scaling = PriceScaling::default();
        let price = price_scaling.calculate_price(PIXEL_MIN_EXPIRATION);
        let price_u128: u128 = price.u128();

        app.execute_contract(
            owner.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata: TileMetadata::default(),
                    updates: vec![PixelUpdate {
                        id: 0,
                        expiration_duration: PIXEL_MIN_EXPIRATION,
                        color,
                    }],
                },
            },
            &[Coin::new(price_u128, NATIVE_DENOM)],
        )
    }

    // High-level helper methods
    pub fn mint_token(
        &self,
        app: &mut App,
        owner: &Addr,
        minter: &Addr,
    ) -> Result<u32, anyhow::Error> {
        // Mint the token and get the response
        let mint_response = self.mint_through_minter(app, owner, minter)?;

        // Extract token_id from the response events
        let token_id = mint_response
            .events
            .iter()
            .find(|e| e.ty == "wasm")
            .and_then(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "token_id")
                    .map(|a| a.value.parse::<u32>().unwrap())
            })
            .expect("Token ID not found in mint response");

        Ok(token_id)
    }
}
