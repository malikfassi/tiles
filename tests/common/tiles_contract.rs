use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_multi_test::Executor;
use sg_multi_test::StargazeApp;
use tiles::msg::{QueryMsg, ExecuteMsg, UpdateConfigMsg, SetPixelColorMsg};
use tiles::state::{Config, PriceScaling};

use crate::common::vending_factory::VendingFactoryContract;

pub struct TilesContract {
    pub contract_addr: Addr,
    pub minter_addr: Addr,
}

impl TilesContract {
    pub fn new(
        _app: &mut StargazeApp,
        _sender: &Addr,
        factory: &VendingFactoryContract,
        collection_addr: Addr,
    ) -> Self {
        Self {
            contract_addr: collection_addr,
            minter_addr: factory.minter_addr.clone(),
        }
    }

    pub fn query_config(&self, app: &StargazeApp) -> Result<Config, Box<dyn std::error::Error>> {
        let msg = QueryMsg::Config {};
        let res: Config = app.wrap().query_wasm_smart(self.contract_addr.clone(), &msg)?;
        Ok(res)
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
        let msg = ExecuteMsg::UpdateConfig(UpdateConfigMsg {
            dev_address,
            dev_fee_percent,
            base_price,
            price_scaling,
        });

        app.execute_contract(sender.clone(), self.contract_addr.clone(), &msg, &[])?;
        Ok(())
    }

    pub fn set_pixel_color(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        msg: SetPixelColorMsg,
        funds: Vec<Coin>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::SetPixelColor(msg);
        app.execute_contract(sender.clone(), self.contract_addr.clone(), &msg, &funds)?;
        Ok(())
    }
} 