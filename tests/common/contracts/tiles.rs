use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::ContractWrapper;

use cw721::NftInfoResponse;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use tiles::contract::msg::{ExecuteMsg, QueryMsg, TileExecuteMsg};
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::Tile;

use crate::common::TestApp;

pub struct TilesContract {
    pub contract_addr: Addr,
}

impl TilesContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    pub fn store_code(app: &mut TestApp) -> Result<u64> {
        let contract = Box::new(ContractWrapper::new(
            tiles::contract::execute,
            tiles::contract::instantiate,
            tiles::contract::query,
        ));
        Ok(app.store_code(contract))
    }

    pub fn update_price_scaling(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        new_price_scaling: PriceScaling,
    ) -> Result<cw_multi_test::AppResponse> {
        self.execute_update_price_scaling(app, sender, new_price_scaling)
    }

    pub fn execute_update_price_scaling(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        new_price_scaling: PriceScaling,
    ) -> Result<cw_multi_test::AppResponse> {
        println!("\n=== Executing Update Price Scaling ===");
        println!("Sender: {}", sender);
        println!("Contract: {}", self.contract_addr);
        println!("New price scaling: {:#?}", new_price_scaling);

        let response = app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::UpdatePriceScaling(new_price_scaling),
            },
            &[],
        );

        match &response {
            Ok(res) => {
                println!("Update successful!");
                println!("Response events: {:#?}", res.events);
            }
            Err(e) => {
                println!("Update failed!");
                println!("Error: {:#?}", e);
            }
        }

        println!("=== Update Price Scaling Complete ===\n");
        response
    }

    pub fn query_token_owner(&self, app: &TestApp, token_id: u32) -> Result<Addr> {
        let owner: String = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Base(Sg721QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: None,
            }),
        )?;
        Ok(Addr::unchecked(owner))
    }

    pub fn query_tile_hash(&self, app: &TestApp, token_id: u32) -> Result<String> {
        let nft_info: NftInfoResponse<Tile> = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Base(Sg721QueryMsg::NftInfo {
                token_id: token_id.to_string(),
            }),
        )?;
        Ok(nft_info.extension.tile_hash)
    }

    pub fn assert_token_owner(&self, app: &TestApp, token_id: u32, expected_owner: &Addr) {
        let actual_owner = self.query_token_owner(app, token_id).unwrap();
        assert_eq!(actual_owner, *expected_owner);
    }

    pub fn query_price_scaling(&self, app: &TestApp) -> Result<PriceScaling> {
        Ok(app
            .inner()
            .wrap()
            .query_wasm_smart(self.contract_addr.clone(), &QueryMsg::PriceScaling {})?)
    }
}
