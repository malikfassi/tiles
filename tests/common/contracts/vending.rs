use crate::common::{constants::CREATION_FEE, test_module::TilesApp as App};
use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::{AppResponse, ContractWrapper, Executor};
use sg2::msg::{CollectionParams, CreateMinterMsg, Sg2ExecuteMsg};
use vending_factory::msg::{InstantiateMsg, VendingMinterInitMsgExtension};

pub struct VendingContract {
    pub address: Option<Addr>,
    pub code_id: Option<u64>,
}

impl VendingContract {
    pub fn new(app: &mut App, _label: &str) -> Self {
        let contract = Box::new(
            ContractWrapper::new(
                vending_factory::contract::execute,
                vending_factory::contract::instantiate,
                vending_factory::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        let code_id = app.store_code(contract);
        Self {
            address: None,
            code_id: Some(code_id),
        }
    }

    pub fn store_code(&self, _app: &mut App) -> Result<u64> {
        Ok(self.code_id.unwrap())
    }

    pub fn instantiate(
        &mut self,
        app: &mut App,
        code_id: u64,
        minter_code_id: u64,
        collection_code_id: u64,
    ) -> Result<Addr> {
        let msg = InstantiateMsg {
            params: vending_factory::state::VendingMinterParams {
                code_id: minter_code_id,
                allowed_sg721_code_ids: vec![collection_code_id],
                frozen: false,
                creation_fee: cosmwasm_std::Coin::new(CREATION_FEE, "ustars"),
                min_mint_price: cosmwasm_std::Coin::new(0, "ustars"),
                mint_fee_bps: 1000,
                max_trading_offset_secs: 60 * 60 * 24 * 7, // 1 week
                extension: vending_factory::state::ParamsExtension {
                    max_token_limit: 10000,
                    max_per_address_limit: 3,
                    airdrop_mint_price: cosmwasm_std::Coin::new(0, "ustars"),
                    airdrop_mint_fee_bps: 0,
                    shuffle_fee: cosmwasm_std::Coin::new(0, "ustars"),
                },
            },
        };
        let addr = app.instantiate_contract(
            code_id,
            Addr::unchecked("creator"),
            &msg,
            &[],
            "vending",
            None,
        )?;
        self.address = Some(addr.clone());
        Ok(addr)
    }

    pub fn create_minter(
        &self,
        app: &mut App,
        owner: &Addr,
        collection_params: CollectionParams,
        init_msg: VendingMinterInitMsgExtension,
    ) -> Result<AppResponse> {
        app.execute_contract(
            owner.clone(),
            self.address.as_ref().unwrap().clone(),
            &Sg2ExecuteMsg::CreateMinter(CreateMinterMsg {
                init_msg,
                collection_params,
            }),
            &[cosmwasm_std::Coin::new(CREATION_FEE, "ustars")],
        )
        .map_err(Into::into)
    }
}
