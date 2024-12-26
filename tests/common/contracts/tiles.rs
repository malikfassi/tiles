use crate::common::test_module::TilesApp as App;
use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::{AppResponse, Executor};
use tiles::contract::msg::{ExecuteMsg, TileExecuteMsg};
use tiles::core::tile::metadata::TileMetadata;
use vending_minter::msg::ExecuteMsg as MinterExecuteMsg;

pub struct TilesContract {
    pub address: Option<Addr>,
}

impl TilesContract {
    pub fn new(address: Addr) -> Self {
        Self {
            address: Some(address),
        }
    }

    pub fn mint_through_minter(
        &self,
        app: &mut App,
        owner: &Addr,
        minter: &Addr,
    ) -> Result<AppResponse> {
        app.execute_contract(
            owner.clone(),
            minter.clone(),
            &MinterExecuteMsg::Mint {},
            &[],
        )
        .map_err(Into::into)
    }

    pub fn update_pixel(
        &self,
        app: &mut App,
        owner: &Addr,
        token_id: u32,
        color: String,
    ) -> Result<AppResponse> {
        app.execute_contract(
            owner.clone(),
            self.address.as_ref().unwrap().clone(),
            &ExecuteMsg::Extension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata: TileMetadata::default(),
                    updates: vec![],
                },
            },
            &[],
        )
        .map_err(Into::into)
    }
}
