use cosmwasm_std::{Addr, Coin};
use cw721::TokensResponse;
use cw_multi_test::Executor;
use sg_multi_test::StargazeApp;

use tiles::contract::msg::{ExecuteMsg, QueryMsg};

pub struct TilesContract {
    pub contract_addr: Addr,
}

impl TilesContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    pub fn set_pixel_color(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        token_id: String,
        color: String,
        position: u32,
        expiration: u64,
        funds: Vec<Coin>,
    ) -> anyhow::Result<()> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &sg721::ExecuteMsg::<tiles::core::tile::Tile, tiles::contract::msg::ExecuteMsg>::Extension {
                msg: tiles::contract::msg::ExecuteMsg::SetPixelColor {
                    token_id,
                    color,
                    position,
                    expiration,
                },
            },
            &funds,
        )?;

        Ok(())
    }

    pub fn update_config(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        dev_address: Option<String>,
        dev_fee_percent: Option<u64>,
        base_price: Option<u128>,
        price_scaling: Option<tiles::contract::state::PriceScaling>,
    ) -> anyhow::Result<()> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &sg721::ExecuteMsg::<tiles::core::tile::Tile, tiles::contract::msg::ExecuteMsg>::Extension {
                msg: tiles::contract::msg::ExecuteMsg::UpdateConfig {
                    dev_address,
                    dev_fee_percent,
                    base_price,
                    price_scaling,
                },
            },
            &[],
        )?;

        Ok(())
    }

    pub fn query_owner_of(
        &self,
        app: &StargazeApp,
        token_id: String,
    ) -> anyhow::Result<String> {
        let res: cw721::OwnerOfResponse = app.wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &sg721_base::msg::QueryMsg::OwnerOf {
                token_id,
                include_expired: None,
            },
        )?;

        Ok(res.owner)
    }

    pub fn query_all_tokens(
        &self,
        app: &StargazeApp,
        owner: String,
    ) -> anyhow::Result<TokensResponse> {
        let res: TokensResponse = app.wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &sg721_base::msg::QueryMsg::Tokens {
                owner,
                start_after: None,
                limit: None,
            },
        )?;

        Ok(res)
    }

    pub fn query_config(&self, app: &StargazeApp) -> anyhow::Result<tiles::contract::state::Config> {
        let res = app.wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &tiles::contract::msg::QueryMsg::Extension(
                tiles::contract::msg::Extension::Config {}
            ),
        )?;

        Ok(res)
    }
}
