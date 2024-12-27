use anyhow::Result;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{ContractWrapper, Executor};
use sg2::msg::{CollectionParams, CreateMinterMsg, Sg2ExecuteMsg};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::{
    AIRDROP_MINT_FEE_BPS, AIRDROP_MINT_PRICE, CREATION_FEE, MAX_PER_ADDRESS_LIMIT, MAX_TOKEN_LIMIT,
    MAX_TRADING_OFFSET_SECS, MINT_FEE_BPS, MIN_MINT_PRICE, SHUFFLE_FEE,
};
use vending_factory::{
    msg::{InstantiateMsg as FactoryInstantiateMsg, VendingMinterInitMsgExtension},
    state::{ParamsExtension, VendingMinterParams},
};

use crate::common::TestApp;

pub struct FactoryContract {
    pub contract_addr: Addr,
    pub label: String,
}

impl FactoryContract {
    pub fn new(_app: &mut TestApp, label: &str) -> Self {
        Self {
            contract_addr: Addr::unchecked("factory111"), // Default address that will be updated
            label: label.to_string(),
        }
    }

    pub fn store_code(&self, app: &mut TestApp) -> Result<u64> {
        let contract = Box::new(ContractWrapper::new(
            vending_factory::contract::execute,
            vending_factory::contract::instantiate,
            vending_factory::contract::query,
        ));
        Ok(app.store_code(contract))
    }

    pub fn instantiate(
        &mut self,
        app: &mut TestApp,
        factory_code_id: u64,
        minter_code_id: u64,
        collection_code_id: u64,
        creator: &Addr,
    ) -> Result<Addr> {
        let msg = FactoryInstantiateMsg {
            params: VendingMinterParams {
                code_id: minter_code_id,
                allowed_sg721_code_ids: vec![collection_code_id],
                frozen: false,
                creation_fee: Coin::new(CREATION_FEE, NATIVE_DENOM),
                min_mint_price: Coin::new(MIN_MINT_PRICE, NATIVE_DENOM),
                mint_fee_bps: MINT_FEE_BPS,
                max_trading_offset_secs: MAX_TRADING_OFFSET_SECS,
                extension: ParamsExtension {
                    max_token_limit: MAX_TOKEN_LIMIT,
                    max_per_address_limit: MAX_PER_ADDRESS_LIMIT,
                    airdrop_mint_price: Coin::new(AIRDROP_MINT_PRICE, NATIVE_DENOM),
                    airdrop_mint_fee_bps: AIRDROP_MINT_FEE_BPS,
                    shuffle_fee: Coin::new(SHUFFLE_FEE, NATIVE_DENOM),
                },
            },
        };
        let addr = app.instantiate_contract(
            factory_code_id,
            creator.clone(),
            &msg,
            &[],
            &self.label,
            None,
        )?;

        self.contract_addr = addr.clone();
        Ok(addr)
    }

    pub fn create_minter(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        collection_params: CollectionParams,
        init_msg: VendingMinterInitMsgExtension,
    ) -> Result<cw_multi_test::AppResponse> {
        app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &Sg2ExecuteMsg::CreateMinter(CreateMinterMsg {
                collection_params,
                init_msg,
            }),
            &[Coin::new(CREATION_FEE, NATIVE_DENOM)],
        )
    }
}
