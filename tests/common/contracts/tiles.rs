use anyhow::Result;
use cosmwasm_std::{Addr, Coin};
use cw721_base::Extension;
use cw_multi_test::Executor;
use sg721::CollectionInfo;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg_std::NATIVE_DENOM;
use tiles::contract::msg::{ExecuteMsg, QueryMsg, TileExecuteMsg};
use tiles::core::pricing::PriceScaling;
use tiles::defaults::constants::MINT_PRICE;
use vending_minter::msg::ExecuteMsg as VendingMinterExecuteMsg;

pub struct TilesContract {
    pub contract_addr: Addr,
}

impl TilesContract {
    pub fn new(contract_addr: Addr) -> Self {
        println!("Creating new TilesContract with address: {}", contract_addr);
        Self { contract_addr }
    }

    pub fn mint_token(
        &self,
        app: &mut crate::common::TestApp,
        owner: &Addr,
        minter: &Addr,
    ) -> Result<u32> {
        // Mint through the minter contract
        let response = app.execute_contract(
            owner.clone(),
            minter.clone(),
            &VendingMinterExecuteMsg::Mint {},
            &[Coin::new(MINT_PRICE, NATIVE_DENOM)],
        )?;

        // Extract token_id from response
        let token_id = response
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

    pub fn query_token_owner(&self, app: &crate::common::TestApp, token_id: u32) -> Result<Addr> {
        let owner: String = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Base(Sg721QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: None,
            }),
        )?;
        Ok(Addr::unchecked(owner))
    }

    pub fn query_tile_hash(&self, app: &crate::common::TestApp, token_id: u32) -> Result<String> {
        let nft_info: cw721::NftInfoResponse<tiles::core::tile::Tile> =
            app.inner().wrap().query_wasm_smart(
                self.contract_addr.clone(),
                &QueryMsg::Base(Sg721QueryMsg::NftInfo {
                    token_id: token_id.to_string(),
                }),
            )?;

        Ok(nft_info.extension.tile_hash)
    }

    pub fn verify_tile_hash(
        &self,
        app: &crate::common::TestApp,
        token_id: u32,
        expected_hash: String,
    ) -> Result<bool> {
        let stored_hash = self.query_tile_hash(app, token_id)?;
        Ok(stored_hash == expected_hash)
    }

    pub fn query_collection_info(
        &self,
        app: &crate::common::TestApp,
    ) -> Result<CollectionInfo<Extension>> {
        let query_msg = sg721_base::msg::QueryMsg::CollectionInfo {};
        Ok(app
            .inner()
            .wrap()
            .query_wasm_smart(self.contract_addr.clone(), &query_msg)?)
    }

    pub fn assert_token_owner(
        &self,
        app: &crate::common::TestApp,
        token_id: u32,
        expected_owner: &Addr,
    ) {
        let owner = self.query_token_owner(app, token_id).unwrap();
        assert_eq!(owner, *expected_owner);
    }

    pub fn update_price_scaling(
        &self,
        app: &mut crate::common::TestApp,
        sender: &Addr,
        price_scaling: PriceScaling,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::UpdatePriceScaling(price_scaling),
            },
            &[],
        )
    }

    pub fn query_price_scaling(&self, app: &crate::common::TestApp) -> Result<PriceScaling> {
        Ok(app
            .inner()
            .wrap()
            .query_wasm_smart(self.contract_addr.clone(), &QueryMsg::PriceScaling {})?)
    }
}
