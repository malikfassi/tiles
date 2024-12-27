use anyhow::Result;
use cosmwasm_std::{coins, Addr, Coin};
use cw721::{NftInfoResponse, OwnerOfResponse};
use cw721_base::Action;
use cw_multi_test::ContractWrapper;
use sg721::{CollectionInfo, RoyaltyInfoResponse, UpdateCollectionInfoMsg};
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg_std::NATIVE_DENOM;
use tiles::contract::msg::{ExecuteMsg, QueryMsg, TileExecuteMsg};
use tiles::core::pricing::PriceScaling;
use tiles::core::tile::{
    metadata::{PixelUpdate, TileMetadata},
    Tile,
};

use crate::common::app::TestApp;

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

    pub fn update_pixel(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
    ) -> Result<cw_multi_test::AppResponse> {
        self.update_pixel_with_metadata(app, sender, token_id, updates, TileMetadata::default())
    }

    pub fn update_pixel_with_metadata(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
        current_metadata: TileMetadata,
    ) -> Result<cw_multi_test::AppResponse> {
        let price_scaling = self.query_price_scaling(app)?;
        let total_price = updates.iter().fold(0u128, |acc, update| {
            acc + price_scaling
                .calculate_price(update.expiration_duration)
                .u128()
        });

        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata,
                    updates,
                },
            },
            &coins(total_price, "ustars"),
        )
    }

    pub fn update_pixel_with_funds(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
        funds_amount: u128,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata: TileMetadata::default(),
                    updates,
                },
            },
            &[Coin::new(funds_amount, NATIVE_DENOM)],
        )
    }

    pub fn query_token_hash(&self, app: &TestApp, token_id: u32) -> Result<String> {
        let response: NftInfoResponse<Tile> = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Base(Sg721QueryMsg::NftInfo {
                token_id: token_id.to_string(),
            }),
        )?;
        Ok(response.extension.tile_hash)
    }

    pub fn query_price_scaling(&self, app: &TestApp) -> Result<PriceScaling> {
        Ok(app
            .inner()
            .wrap()
            .query_wasm_smart(self.contract_addr.clone(), &QueryMsg::PriceScaling {})?)
    }

    pub fn assert_token_owner(&self, app: &TestApp, token_id: u32, expected_owner: &Addr) {
        let response: OwnerOfResponse = app
            .inner()
            .wrap()
            .query_wasm_smart(
                self.contract_addr.clone(),
                &QueryMsg::Base(Sg721QueryMsg::OwnerOf {
                    token_id: token_id.to_string(),
                    include_expired: None,
                }),
            )
            .unwrap();
        assert_eq!(response.owner, expected_owner.to_string());
    }

    pub fn execute_transfer_nft(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        recipient: &Addr,
        token_id: String,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::TransferNft {
                recipient: recipient.to_string(),
                token_id,
            },
            &[],
        )
    }

    pub fn execute_approve(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        spender: &Addr,
        token_id: String,
        expires: Option<cw721::Expiration>,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Approve {
                spender: spender.to_string(),
                token_id,
                expires,
            },
            &[],
        )
    }

    pub fn execute_revoke(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        spender: &Addr,
        token_id: String,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Revoke {
                spender: spender.to_string(),
                token_id,
            },
            &[],
        )
    }

    pub fn execute_burn(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: String,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Burn { token_id },
            &[],
        )
    }

    pub fn execute_update_ownership(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        action: Action,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::UpdateOwnership(action),
            &[],
        )
    }

    pub fn execute_update_collection_info(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        collection_info: UpdateCollectionInfoMsg<RoyaltyInfoResponse>,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::UpdateCollectionInfo { collection_info },
            &[],
        )
    }

    pub fn execute_freeze_collection_info(
        &self,
        app: &mut TestApp,
        sender: &Addr,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::FreezeCollectionInfo,
            &[],
        )
    }

    pub fn query_owner_of(&self, app: &TestApp, token_id: String) -> Result<OwnerOfResponse> {
        Ok(app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Base(Sg721QueryMsg::OwnerOf {
                token_id,
                include_expired: None,
            }),
        )?)
    }

    pub fn query_collection_info(
        &self,
        app: &TestApp,
    ) -> Result<CollectionInfo<RoyaltyInfoResponse>> {
        Ok(app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Base(Sg721QueryMsg::CollectionInfo {}),
        )?)
    }
}
