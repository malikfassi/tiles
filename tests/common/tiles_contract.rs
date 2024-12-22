use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw721::TokensResponse;
use cw_multi_test::Executor;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg_multi_test::StargazeApp;

use tiles::msg::{ExecuteMsg, QueryMsg, SetPixelColorMsg, UpdateConfigMsg};
use tiles::state::{Config, PriceScaling};

pub struct TilesContract {
    pub contract_addr: Addr,
}

impl TilesContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    pub fn update_config(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        dev_address: Option<String>,
        dev_fee_percent: Option<Decimal>,
        base_price: Option<Uint128>,
        price_scaling: Option<PriceScaling>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::UpdateConfig(UpdateConfigMsg {
                dev_address,
                dev_fee_percent,
                base_price,
                price_scaling,
            }),
            &[],
        )?;
        Ok(())
    }

    pub fn set_pixel_color(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        msg: SetPixelColorMsg,
        funds: Vec<Coin>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::SetPixelColor(msg),
            &funds,
        )?;
        Ok(())
    }

    pub fn query_config(&self, app: &StargazeApp) -> Result<Config, Box<dyn std::error::Error>> {
        let config: Config = app
            .wrap()
            .query_wasm_smart(self.contract_addr.clone(), &QueryMsg::Config {})?;
        Ok(config)
    }

    pub fn query_owner_of(
        &self,
        app: &StargazeApp,
        token_id: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let owner: String = app.wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Sg721Base(Sg721QueryMsg::OwnerOf {
                token_id,
                include_expired: None,
            }),
        )?;
        Ok(owner)
    }

    pub fn query_tokens(
        &self,
        app: &StargazeApp,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response: TokensResponse = app.wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Sg721Base(Sg721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            }),
        )?;
        Ok(response.tokens)
    }
}
