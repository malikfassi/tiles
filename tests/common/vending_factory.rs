use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};
use sg2::msg::{CreateMinterMsg, MinterParams};
use sg2::state::ParamsExtension;
use vending_factory::msg::InstantiateMsg;
use vending_minter::contract;

use tiles::defaults::config::{
    DEFAULT_CREATION_FEE, DEFAULT_MAX_PER_ADDRESS_LIMIT, DEFAULT_MAX_TOKEN_LIMIT,
    DEFAULT_MIN_MINT_PRICE, DEFAULT_MINT_FEE_BPS, DEFAULT_SHUFFLE_FEE,
};

pub struct VendingFactory {
    pub address: Addr,
}

impl VendingFactory {
    pub fn new(app: &mut App, sender: &Addr) -> Self {
        // Store code
        let factory_code = ContractWrapper::new(
            vending_factory::contract::execute,
            vending_factory::contract::instantiate,
            vending_factory::contract::query,
        )
        .with_reply(contract::reply);
        let factory_code_id = app.store_code(Box::new(factory_code));

        let minter_code = ContractWrapper::new(
            contract::execute,
            contract::instantiate,
            contract::query,
        )
        .with_reply(contract::reply);
        let minter_code_id = app.store_code(Box::new(minter_code));

        // Instantiate factory
        let msg = InstantiateMsg {
            params: MinterParams {
                code_id: minter_code_id,
                creation_fee: Coin::new(DEFAULT_CREATION_FEE, "ustars"),
                min_mint_price: Coin::new(DEFAULT_MIN_MINT_PRICE, "ustars"),
                mint_fee_bps: DEFAULT_MINT_FEE_BPS,
                max_trading_offset_secs: 0,
                allowed_sg721_code_ids: vec![],
                frozen: false,
                extension: ParamsExtension {
                    max_token_limit: DEFAULT_MAX_TOKEN_LIMIT,
                    max_per_address_limit: DEFAULT_MAX_PER_ADDRESS_LIMIT,
                    airdrop_mint_price: Coin::new(0, "ustars"),
                    airdrop_mint_fee_bps: 0,
                    shuffle_fee: Coin::new(DEFAULT_SHUFFLE_FEE, "ustars"),
                },
            },
        };

        let factory_addr = app
            .instantiate_contract(
                factory_code_id,
                sender.clone(),
                &msg,
                &[],
                "vending factory",
                None,
            )
            .unwrap();

        Self {
            address: factory_addr,
        }
    }

    pub fn create_minter(
        &self,
        app: &mut App,
        sender: &Addr,
        msg: CreateMinterMsg,
    ) -> Result<(Addr, Addr), Box<dyn std::error::Error>> {
        // Execute create minter
        let res = app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &msg,
            &[],
        )?;

        // Parse response attributes
        let minter_addr = res
            .events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "_contract_address")
                    .map(|a| Addr::unchecked(a.value.clone()))
            })
            .ok_or("Failed to find minter address")?;

        let collection_addr = res
            .events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "_contract_address")
                    .map(|a| Addr::unchecked(a.value.clone()))
            })
            .ok_or("Failed to find collection address")?;

        Ok((minter_addr, collection_addr))
    }
}
