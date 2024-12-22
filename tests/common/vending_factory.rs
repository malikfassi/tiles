use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{ContractWrapper, Executor};
use sg_multi_test::StargazeApp;
use vending_factory::msg::VendingMinterCreateMsg;
use std::fmt;

use crate::common::NATIVE_DENOM;

#[derive(Debug)]
pub enum VendingFactoryError {
    ExecuteError(String),
    MinterAddressNotFound,
    CollectionAddressNotFound,
}

impl fmt::Display for VendingFactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VendingFactoryError::ExecuteError(msg) => write!(f, "Execute error: {}", msg),
            VendingFactoryError::MinterAddressNotFound => write!(f, "Minter address not found in events"),
            VendingFactoryError::CollectionAddressNotFound => write!(f, "Collection address not found in events"),
        }
    }
}

impl std::error::Error for VendingFactoryError {}

pub struct VendingFactoryContract {
    pub contract_addr: Addr,
    pub minter_addr: Addr,
    pub sg721_code_id: u64,
}

impl VendingFactoryContract {
    pub fn new(app: &mut StargazeApp, sender: &Addr) -> Self {
        println!("Storing contract codes...");
        // Store contract codes
        let factory_code = ContractWrapper::new(
            vending_factory::contract::execute,
            vending_factory::contract::instantiate,
            vending_factory::contract::query,
        );
        let factory_code_id = app.store_code(Box::new(factory_code));
        println!("Factory code ID: {}", factory_code_id);

        let minter_code = ContractWrapper::new(
            vending_minter::contract::execute,
            vending_minter::contract::instantiate,
            vending_minter::contract::query,
        )
        .with_reply(vending_minter::contract::reply);
        let minter_code_id = app.store_code(Box::new(minter_code));
        println!("Minter code ID: {}", minter_code_id);

        // Use our contract code
        let sg721_code = ContractWrapper::new(
            tiles::contract::execute,
            tiles::contract::instantiate,
            tiles::contract::query,
        );
        let sg721_code_id = app.store_code(Box::new(sg721_code));
        println!("SG721 code ID: {}", sg721_code_id);

        println!("Instantiating factory...");
        // Instantiate factory
        let factory_addr = app
            .instantiate_contract(
                factory_code_id,
                sender.clone(),
                &vending_factory::msg::InstantiateMsg {
                    params: vending_factory::state::VendingMinterParams {
                        code_id: minter_code_id,
                        allowed_sg721_code_ids: vec![sg721_code_id],
                        frozen: false,
                        creation_fee: Coin::new(1_000_000, NATIVE_DENOM),
                        min_mint_price: Coin::new(100_000, NATIVE_DENOM),
                        mint_fee_bps: 1000,
                        max_trading_offset_secs: 60 * 60 * 24 * 7, // 1 week
                        extension: vending_factory::state::ParamsExtension {
                            max_token_limit: 10000,
                            max_per_address_limit: 50,
                            airdrop_mint_price: Coin::new(0, NATIVE_DENOM),
                            airdrop_mint_fee_bps: 0,
                            shuffle_fee: Coin::new(500_000, NATIVE_DENOM),
                        },
                    },
                },
                &[],
                "vending_factory",
                None,
            )
            .map_err(|e| {
                println!("Failed to instantiate factory: {}", e);
                e
            })
            .unwrap();
        println!("Factory instantiated at address: {}", factory_addr);

        Self {
            contract_addr: factory_addr,
            minter_addr: Addr::unchecked(""),
            sg721_code_id,
        }
    }

    pub fn create_minter(
        &mut self,
        app: &mut StargazeApp,
        sender: &Addr,
        mut msg: VendingMinterCreateMsg,
    ) -> Result<(Addr, Addr), Box<dyn std::error::Error>> {
        // Update collection code ID
        msg.collection_params.code_id = self.sg721_code_id;
        println!("Creating minter with collection code ID: {}", self.sg721_code_id);

        println!("Executing CreateMinter message...");
        let res = app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &vending_factory::msg::ExecuteMsg::CreateMinter(msg),
            &[Coin::new(1_000_000, NATIVE_DENOM)],
        )
        .map_err(|e| {
            println!("Failed to execute CreateMinter: {}", e);
            VendingFactoryError::ExecuteError(e.to_string())
        })?;
        println!("CreateMinter message executed");

        // Extract minter and collection addresses from events
        let mut minter_addr = None;
        let mut collection_addr = None;

        println!("Processing events...");
        for event in res.events {
            println!("Event type: {}", event.ty);
            for attr in &event.attributes {
                println!("  {}: {}", attr.key, attr.value);
                
                if event.ty == "instantiate" && attr.key == "_contract_addr" {
                    if minter_addr.is_none() {
                        println!("Found minter address: {}", attr.value);
                        minter_addr = Some(attr.value.to_string());
                    } else if collection_addr.is_none() {
                        println!("Found collection address: {}", attr.value);
                        collection_addr = Some(attr.value.to_string());
                    }
                } else if event.ty == "wasm" && attr.key == "sg721_address" {
                    println!("Found collection address from sg721_address: {}", attr.value);
                    collection_addr = Some(attr.value.to_string());
                }
            }
        }

        let minter_addr = Addr::unchecked(
            minter_addr.ok_or(VendingFactoryError::MinterAddressNotFound)?,
        );
        let collection_addr = Addr::unchecked(
            collection_addr.ok_or(VendingFactoryError::CollectionAddressNotFound)?,
        );

        println!("Minter created at: {}", minter_addr);
        println!("Collection created at: {}", collection_addr);

        self.minter_addr = minter_addr.clone();

        Ok((minter_addr, collection_addr))
    }
} 