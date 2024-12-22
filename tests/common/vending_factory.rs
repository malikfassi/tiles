use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{ContractWrapper, Executor};
use sg_multi_test::StargazeApp;
use vending_factory::msg::VendingMinterCreateMsg;

use crate::common::NATIVE_DENOM;

pub struct VendingFactoryContract {
    pub contract_addr: Addr,
    pub minter_addr: Addr,
    pub sg721_code_id: u64,
}

impl VendingFactoryContract {
    pub fn new(app: &mut StargazeApp, sender: &Addr) -> Self {
        // Store contract codes
        let factory_code = ContractWrapper::new(
            vending_factory::contract::execute,
            vending_factory::contract::instantiate,
            vending_factory::contract::query,
        );
        let factory_code_id = app.store_code(Box::new(factory_code));

        let minter_code = ContractWrapper::new(
            vending_minter::contract::execute,
            vending_minter::contract::instantiate,
            vending_minter::contract::query,
        )
        .with_reply(vending_minter::contract::reply);
        let minter_code_id = app.store_code(Box::new(minter_code));

        let sg721_code = ContractWrapper::new(
            tiles::contract::execute,
            tiles::contract::instantiate,
            tiles::contract::query,
        );
        let sg721_code_id = app.store_code(Box::new(sg721_code));

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
            .unwrap();

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

        let res = app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &vending_factory::msg::ExecuteMsg::CreateMinter(msg),
            &[Coin::new(1_000_000, NATIVE_DENOM)],
        )?;

        // Extract minter and collection addresses from events
        let mut minter_addr = None;
        let mut collection_addr = None;

        for event in res.events {
            println!("Event type: {}", event.ty);
            for attr in &event.attributes {
                println!("  {}: {}", attr.key, attr.value);
                
                match (event.ty.as_str(), attr.key.as_str(), attr.value.as_str()) {
                    ("instantiate", "_contract_address", addr) if minter_addr.is_none() => {
                        minter_addr = Some(addr.to_string());
                    }
                    ("instantiate", "_contract_address", addr) if collection_addr.is_none() => {
                        collection_addr = Some(addr.to_string());
                    }
                    _ => {}
                }
            }
        }

        let minter_addr = Addr::unchecked(minter_addr.ok_or("Failed to extract minter address")?);
        let collection_addr = Addr::unchecked(collection_addr.ok_or("Failed to extract collection address")?);

        self.minter_addr = minter_addr.clone();

        Ok((minter_addr, collection_addr))
    }
} 