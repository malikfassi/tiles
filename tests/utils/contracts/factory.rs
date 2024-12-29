use anyhow::Result;
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};
use sg2::msg::{CollectionParams, CreateMinterMsg};
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::{
    AIRDROP_MINT_FEE_BPS, AIRDROP_MINT_PRICE, BASE_TOKEN_URI, COLLECTION_DESCRIPTION,
    COLLECTION_NAME, COLLECTION_SYMBOL, COLLECTION_URI, CREATION_FEE, DEFAULT_ROYALTY_SHARE,
    MAX_PER_ADDRESS_LIMIT, MAX_TOKEN_LIMIT, MAX_TRADING_OFFSET_SECS, MINT_FEE_BPS, MINT_PRICE,
    MIN_MINT_PRICE, SHUFFLE_FEE, TWENTY_FOUR_HOURS,
};

use vending_factory::{
    msg::{
        ExecuteMsg as FactoryExecuteMsg, InstantiateMsg as FactoryInstantiateMsg,
        VendingMinterInitMsgExtension,
    },
    state::{ParamsExtension, VendingMinterParams},
};

use crate::utils::app::TestApp;

#[derive(Clone)]
pub struct FactoryContract {
    pub contract_addr: Addr,
    pub label: String,
    pub minter_code_id: Option<u64>,
    pub collection_code_id: Option<u64>,
}

impl FactoryContract {
    pub fn new(_app: &mut TestApp, label: &str) -> Self {
        Self {
            contract_addr: Addr::unchecked("factory111"), // Default address that will be updated
            label: label.to_string(),
            minter_code_id: None,
            collection_code_id: None,
        }
    }

    pub fn store_code(app: &mut TestApp) -> Result<u64> {
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
    ) -> Result<(Addr, cw_multi_test::AppResponse)> {
        self.minter_code_id = Some(minter_code_id);
        self.collection_code_id = Some(collection_code_id);
        let msg = FactoryInstantiateMsg {
            params: VendingMinterParams {
                code_id: minter_code_id,
                allowed_sg721_code_ids: vec![collection_code_id],
                frozen: false,
                creation_fee: Coin::new(CREATION_FEE, NATIVE_DENOM),
                min_mint_price: Coin::new(MIN_MINT_PRICE, NATIVE_DENOM),
                mint_fee_bps: MINT_FEE_BPS,
                max_trading_offset_secs: 0,
                extension: ParamsExtension {
                    max_token_limit: MAX_TOKEN_LIMIT,
                    max_per_address_limit: MAX_PER_ADDRESS_LIMIT,
                    airdrop_mint_price: Coin::new(AIRDROP_MINT_PRICE, NATIVE_DENOM),
                    airdrop_mint_fee_bps: AIRDROP_MINT_FEE_BPS,
                    shuffle_fee: Coin::new(SHUFFLE_FEE, NATIVE_DENOM),
                },
            },
        };
        let addr = app.inner_mut().instantiate_contract(
            factory_code_id,
            creator.clone(),
            &msg,
            &[],
            &self.label,
            None,
        )?;
        self.contract_addr = addr.clone();
        Ok((addr, cw_multi_test::AppResponse::default()))
    }

    pub fn create_minter(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        collection_params: CollectionParams,
        init_msg: VendingMinterInitMsgExtension,
    ) -> Result<cw_multi_test::AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &FactoryExecuteMsg::CreateMinter(CreateMinterMsg {
                collection_params,
                init_msg,
            }),
            &[Coin::new(CREATION_FEE, NATIVE_DENOM)],
        )
    }

    pub fn create_test_minter(
        &self,
        app: &mut TestApp,
        creator: &Addr,
    ) -> Result<(Addr, Addr, AppResponse)> {
        // Use the collection code ID from factory initialization
        let collection_code_id = self.collection_code_id.expect("Collection code ID not set");

        let collection_params = CollectionParams {
            code_id: collection_code_id,
            name: COLLECTION_NAME.to_string(),
            symbol: COLLECTION_SYMBOL.to_string(),
            info: CollectionInfo {
                creator: creator.to_string(),
                description: COLLECTION_DESCRIPTION.to_string(),
                image: COLLECTION_URI.to_string(),
                external_link: None,
                explicit_content: Some(false),
                start_trading_time: None,
                royalty_info: Some(RoyaltyInfoResponse {
                    payment_address: creator.to_string(),
                    share: Decimal::percent(DEFAULT_ROYALTY_SHARE),
                }),
            },
        };

        let block_time = app.inner().block_info().time;
        println!("\nCreating minter with parameters:");
        println!("Block time: {}", block_time);

        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: BASE_TOKEN_URI.to_string(),
            payment_address: Some(creator.to_string()),
            start_time: block_time, // Start immediately
            num_tokens: MAX_TOKEN_LIMIT,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: MAX_PER_ADDRESS_LIMIT,
            whitelist: None,
        };
        println!("Start time: {}", init_msg.start_time);
        println!(
            "Mint price: {} {}",
            init_msg.mint_price.amount, init_msg.mint_price.denom
        );
        println!("Per address limit: {}", init_msg.per_address_limit);

        let response = self.create_minter(app, creator, collection_params, init_msg)?;

        // Debug print all events
        println!("\nAll events:");
        for (i, event) in response.events.iter().enumerate() {
            println!("\nEvent {}: {}", i, event.ty);
            for attr in &event.attributes {
                println!("  {} = {}", attr.key, attr.value);
            }
        }

        // Extract contract addresses
        let minter_code_id = self.minter_code_id.expect("Minter code ID not set");
        let minter_addr = response
            .events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| {
                let code_id_attr = e.attributes.iter().find(|a| a.key == "code_id")?;
                println!(
                    "\nFound instantiate event with code_id: {}",
                    code_id_attr.value
                );
                if code_id_attr.value == minter_code_id.to_string() {
                    let addr = e.attributes.iter().find(|a| a.key == "_contract_addr");
                    println!("Found minter address: {:?}", addr.map(|a| &a.value));
                    addr
                } else {
                    println!(
                        "Code ID didn't match expected minter code ID {}",
                        minter_code_id
                    );
                    None
                }
            })
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("Minter address not found in events");

        let sg721_addr = response
            .events
            .iter()
            .find(|e| {
                e.ty == "wasm"
                    && e.attributes
                        .iter()
                        .any(|a| a.key == "action" && a.value == "instantiate_sg721_reply")
            })
            .and_then(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "sg721_address")
                    .map(|a| Addr::unchecked(a.value.clone()))
            })
            .expect("SG721 address not found in events");

        Ok((minter_addr, sg721_addr, response))
    }
}
