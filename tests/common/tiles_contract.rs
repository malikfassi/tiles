use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_multi_test::Executor;
use sg_multi_test::StargazeApp;

use tiles::msg::{ExecuteMsg, QueryMsg, UpdateConfigMsg, SetPixelColorMsg};
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
} 