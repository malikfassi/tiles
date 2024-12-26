use crate::common::app::TestApp;
use anyhow::Result;
use cosmwasm_std::{Addr, Coin};
use cw721::NftInfoResponse;
use cw_multi_test::AppResponse;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg_std::NATIVE_DENOM;
use tiles::contract::msg::{ExecuteMsg, TileExecuteMsg};
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::{
    metadata::{PixelUpdate, TileMetadata},
    Tile,
};
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
        app: &mut TestApp,
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
        app: &mut TestApp,
        owner: &Addr,
        token_id: u32,
        color: String,
    ) -> Result<AppResponse, anyhow::Error> {
        let price_scaling = PriceScaling::default();
        let price = price_scaling.calculate_price(PIXEL_MIN_EXPIRATION);
        let price_u128: u128 = price.u128();

        let mut metadata = TileMetadata::default();

        // Execute update through the tiles contract
        app.execute_contract(
            owner.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata: metadata,
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

    // Query helpers
    pub fn query_token_owner(&self, app: &TestApp, token_id: u32) -> Result<Addr, anyhow::Error> {
        let owner: String = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &Sg721QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: None,
            },
        )?;
        Ok(Addr::unchecked(owner))
    }

    pub fn query_tile_hash(&self, app: &TestApp, token_id: u32) -> Result<String, anyhow::Error> {
        let nft_info: NftInfoResponse<Tile> = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &Sg721QueryMsg::NftInfo {
                token_id: token_id.to_string(),
            },
        )?;

        Ok(nft_info.extension.tile_hash)
    }

    pub fn verify_tile_hash(
        &self,
        app: &TestApp,
        token_id: u32,
        expected_hash: String,
    ) -> Result<bool, anyhow::Error> {
        let stored_hash = self.query_tile_hash(app, token_id)?;
        Ok(stored_hash == expected_hash)
    }

    // High-level helper methods
    pub fn mint_token(
        &self,
        app: &mut TestApp,
        owner: &Addr,
        minter: &Addr,
    ) -> Result<u32, anyhow::Error> {
        let mint_response = self.mint_through_minter(app, owner, minter)?;

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
