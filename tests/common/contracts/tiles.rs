use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, StdError, StdResult};
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg_multi_test::StargazeApp as App;
use sg_multi_test::{ContractWrapper, Executor};

use tiles::{
    contract::{
        execute, instantiate,
        msg::{CustomExecuteMsg, ExecuteMsg, QueryMsg},
        query,
    },
    core::{
        pricing::PriceScaling,
        tile::{
            metadata::{PixelUpdate, TileMetadata},
            Tile,
        },
    },
};

pub struct TilesContract {
    pub addr: Addr,
}

impl TilesContract {
    pub fn store_code(app: &mut App) -> StdResult<u64> {
        println!("Creating tiles contract wrapper...");
        let contract = ContractWrapper::new(execute, instantiate, query);
        println!("Storing tiles code...");
        let code_id = app.store_code(Box::new(contract));
        println!("✓ Tiles code stored successfully");
        Ok(code_id)
    }

    pub fn new(addr: Addr) -> Self {
        Self { addr }
    }

    pub fn mint(
        &self,
        app: &mut App,
        sender: &Addr,
        token_id: String,
        token_uri: Option<String>,
        extension: Option<Tile>,
    ) -> StdResult<()> {
        let extension = extension.unwrap_or_else(|| Tile {
            tile_hash: TileMetadata::default().hash(),
        });

        let msg = ExecuteMsg::Base(sg721::ExecuteMsg::Mint {
            token_id,
            owner: sender.to_string(),
            token_uri,
            extension,
        });

        app.execute_contract(sender.clone(), self.addr.clone(), &msg, &[])
            .map_err(|e| StdError::generic_err(e.to_string()))
            .map(|_| ())
    }

    pub fn update_config(
        &self,
        app: &mut App,
        sender: &Addr,
        dev_address: Option<String>,
        dev_fee_percent: Option<cosmwasm_std::Decimal>,
        price_scaling: Option<PriceScaling>,
    ) -> StdResult<()> {
        let msg = ExecuteMsg::Custom(CustomExecuteMsg::UpdateConfig {
            tile_royalty_payment_address: dev_address,
            tile_royalty_fee_percent: dev_fee_percent,
            price_scaling,
        });

        app.execute_contract(sender.clone(), self.addr.clone(), &msg, &[])
            .map_err(|e| StdError::generic_err(e.to_string()))
            .map(|_| ())
    }

    pub fn set_pixel_color(
        &self,
        app: &mut App,
        sender: &Addr,
        token_id: String,
        current_metadata: TileMetadata,
        updates: Vec<PixelUpdate>,
        funds: Vec<Coin>,
    ) -> StdResult<()> {
        let msg = ExecuteMsg::Custom(CustomExecuteMsg::SetPixelColor {
            token_id,
            current_metadata,
            updates,
        });

        app.execute_contract(sender.clone(), self.addr.clone(), &msg, &funds)
            .map_err(|e| StdError::generic_err(e.to_string()))
            .map(|_| ())
    }

    pub fn query_token(&self, app: &App, token_id: String) -> StdResult<TokenResponse> {
        let msg = QueryMsg::Base(Sg721QueryMsg::NftInfo { token_id });
        app.wrap()
            .query_wasm_smart(self.addr.clone(), &msg)
            .map_err(|e| StdError::generic_err(e.to_string()))
    }
}

#[cw_serde]
pub struct TokenResponse {
    pub token_uri: Option<String>,
    pub extension: Tile,
}
