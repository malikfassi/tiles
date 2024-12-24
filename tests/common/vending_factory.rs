use anyhow::Result;
use cosmwasm_std::{Addr, Coin, Empty, Timestamp};
use cw_multi_test::{ContractWrapper, Executor};
use sg721::CollectionInfo;
use sg_multi_test::StargazeApp;
use sg2::MinterParams as InstantiateParams;
use vending_factory::msg::{
    VendingMinterCreateMsg, 
    VendingMinterInitMsgExtension,
    ExecuteMsg as VendingFactoryExecuteMsg,
    InstantiateMsg as VendingFactoryInstantiateMsg,
};
use vending_minter::msg::ExecuteMsg as VendingMinterExecuteMsg;

use crate::common::NATIVE_DENOM;

pub struct VendingFactory {
    pub contract_addr: Addr,
    pub sg721_code_id: u64,
}

pub fn create_vending_factory(app: &mut StargazeApp, sender: &Addr) -> Result<VendingFactory> {
    // Store sg721 code
    let sg721_code_id = app.store_code(Box::new(ContractWrapper::new(
        sg721_base::entry::execute,
        sg721_base::entry::instantiate,
        sg721_base::entry::query,
    )));

    // Store vending minter code
    let vending_minter_code_id = app.store_code(Box::new(ContractWrapper::new(
        vending_minter::contract::execute,
        vending_minter::contract::instantiate,
        vending_minter::contract::query,
    ).with_reply(vending_minter::contract::reply)));

    // Store vending factory code
    let vending_factory_code_id = app.store_code(Box::new(ContractWrapper::new(
        vending_factory::contract::execute,
        vending_factory::contract::instantiate,
        vending_factory::contract::query,
    )));

    // Instantiate vending factory
    let factory_addr = app.instantiate_contract(
        vending_factory_code_id,
        sender.clone(),
        &VendingFactoryInstantiateMsg {
            params: InstantiateParams {
                code_id: vending_minter_code_id,
                allowed_sg721_code_ids: vec![sg721_code_id],
                frozen: false,
                creation_fee: Coin::new(1_000_000, NATIVE_DENOM),
                min_mint_price: Coin::new(100_000, NATIVE_DENOM),
                mint_fee_bps: 1000,
                max_trading_offset_secs: 60 * 60 * 24 * 7, // 1 week
                extension: vending_factory::state::ParamsExtension {
                    max_token_limit: 10000,
                    max_per_address_limit: 50,
                    airdrop_mint_price: Coin::new(100_000, NATIVE_DENOM),
                    airdrop_mint_fee_bps: 1000,
                    shuffle_fee: Coin::new(500_000, NATIVE_DENOM),
                },
            },
        },
        &[],
        "vending-factory",
        None,
    )?;

    Ok(VendingFactory {
        contract_addr: factory_addr,
        sg721_code_id,
    })
}

impl VendingFactory {
    pub fn create_minter(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        name: String,
        symbol: String,
        description: String,
        image: String,
        base_token_uri: String,
        num_tokens: u32,
        mint_price: Coin,
        per_address_limit: u32,
        start_time: Option<u64>,
        payment_address: Option<String>,
        whitelist: Option<String>,
        collection_admin: Option<String>
    ) -> Result<(Addr, Addr)> {
        let start_time = start_time.map(|t| Timestamp::from_seconds(t))
            .unwrap_or_else(|| app.block_info().time);

        let create_msg = VendingMinterCreateMsg {
            init_msg: VendingMinterInitMsgExtension {
                base_token_uri,
                payment_address,
                start_time,
                num_tokens,
                mint_price,
                per_address_limit,
                whitelist,
            },
            collection_params: sg2::msg::CollectionParams {
                code_id: self.sg721_code_id,
                name,
                symbol,
                info: CollectionInfo {
                    creator: self.contract_addr.to_string(),
                    description,
                    image,
                    external_link: None,
                    explicit_content: None,
                    start_trading_time: None,
                    royalty_info: None,
                },
            },
        };

        let res = app.execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &VendingFactoryExecuteMsg::CreateMinter(create_msg),
            &[Coin::new(1_000_000, NATIVE_DENOM)],
        )?;

        // Print events for debugging
        println!("Events: {:?}", res.events);

        // Extract addresses from wasm events
        let mut minter_addr = None;
        let mut collection_addr = None;

        for event in &res.events {
            println!("Event type: {}", event.ty);
            for attr in &event.attributes {
                println!("Attribute: {} = {}", attr.key, attr.value);
                match (event.ty.as_str(), attr.key.as_str(), attr.value.as_str()) {
                    ("wasm", "minter", addr) => minter_addr = Some(Addr::unchecked(addr)),
                    ("wasm", "sg721_address", addr) => collection_addr = Some(Addr::unchecked(addr)),
                    _ => {}
                }
            }
        }

        let minter_addr = minter_addr.ok_or_else(|| anyhow::anyhow!("Minter address not found"))?;
        let collection_addr = collection_addr.ok_or_else(|| anyhow::anyhow!("Collection address not found"))?;

        println!("Found addresses - Minter: {}, Collection: {}", minter_addr, collection_addr);

        // Set collection admin if provided (optional)
        if let Some(admin) = collection_admin {
            // Transfer ownership using the minter contract
            app.execute_contract(
                minter_addr.clone(),
                collection_addr.clone(),
                &sg721_base::msg::ExecuteMsg::<Empty, Empty>::UpdateOwnership(cw721_base::Action::TransferOwnership {
                    new_owner: admin.clone(),
                    expiry: None,
                }),
                &[],
            )?;

            // Accept ownership as the new owner
            app.execute_contract(
                Addr::unchecked(&admin),
                collection_addr.clone(),
                &sg721_base::msg::ExecuteMsg::<Empty, Empty>::UpdateOwnership(cw721_base::Action::AcceptOwnership {}),
                &[],
            )?;
        }

        Ok((minter_addr, collection_addr))
    }

    pub fn mint_token(
        &self,
        app: &mut StargazeApp,
        sender: &Addr,
        minter_addr: &Addr,
    ) -> Result<()> {
        app.execute_contract(
            sender.clone(),
            minter_addr.clone(),
            &VendingMinterExecuteMsg::Mint {},
            &[Coin::new(100_000_000, NATIVE_DENOM)],
        )?;

        Ok(())
    }
}
