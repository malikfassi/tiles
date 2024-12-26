use crate::common::app::TestApp;
use anyhow::Result;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{AppResponse, ContractWrapper};
use sg2::msg::{CollectionParams, CreateMinterMsg, Sg2ExecuteMsg};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::{
    AIRDROP_MINT_FEE_BPS, AIRDROP_MINT_PRICE, CREATION_FEE, MAX_PER_ADDRESS_LIMIT, MAX_TOKEN_LIMIT,
    MAX_TRADING_OFFSET_SECS, MINT_FEE_BPS, MIN_MINT_PRICE, SHUFFLE_FEE,
};
use vending_factory::msg::{InstantiateMsg, VendingMinterInitMsgExtension};

pub struct VendingContract {
    pub contract_addr: Option<Addr>,
}

impl VendingContract {
    pub fn new(app: &mut TestApp, _label: &str) -> Self {
        let contract = Box::new(
            ContractWrapper::new(
                vending_factory::contract::execute,
                vending_factory::contract::instantiate,
                vending_factory::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        let _code_id = app.store_code(contract);
        Self {
            contract_addr: None,
        }
    }

    pub fn store_code(&self, app: &mut TestApp) -> Result<u64> {
        let contract = Box::new(
            ContractWrapper::new(
                vending_factory::contract::execute,
                vending_factory::contract::instantiate,
                vending_factory::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        Ok(app.store_code(contract))
    }

    pub fn instantiate(
        &mut self,
        app: &mut TestApp,
        code_id: u64,
        minter_code_id: u64,
        collection_code_id: u64,
    ) -> Result<Addr> {
        let msg = InstantiateMsg {
            params: vending_factory::state::VendingMinterParams {
                code_id: minter_code_id,
                allowed_sg721_code_ids: vec![collection_code_id],
                frozen: false,
                creation_fee: Coin::new(CREATION_FEE, NATIVE_DENOM),
                min_mint_price: Coin::new(MIN_MINT_PRICE, NATIVE_DENOM),
                mint_fee_bps: MINT_FEE_BPS,
                max_trading_offset_secs: MAX_TRADING_OFFSET_SECS,
                extension: vending_factory::state::ParamsExtension {
                    max_token_limit: MAX_TOKEN_LIMIT,
                    max_per_address_limit: MAX_PER_ADDRESS_LIMIT,
                    airdrop_mint_price: Coin::new(AIRDROP_MINT_PRICE, NATIVE_DENOM),
                    airdrop_mint_fee_bps: AIRDROP_MINT_FEE_BPS,
                    shuffle_fee: Coin::new(SHUFFLE_FEE, NATIVE_DENOM),
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
        self.contract_addr = Some(addr.clone());
        Ok(addr)
    }

    pub fn create_minter(
        &self,
        app: &mut TestApp,
        owner: &Addr,
        collection_params: CollectionParams,
        init_msg: VendingMinterInitMsgExtension,
    ) -> Result<AppResponse> {
        app.execute_contract(
            owner.clone(),
            self.contract_addr.as_ref().unwrap().clone(),
            &Sg2ExecuteMsg::CreateMinter(CreateMinterMsg {
                init_msg,
                collection_params,
            }),
            &[Coin::new(CREATION_FEE, NATIVE_DENOM)],
        )
    }
}
