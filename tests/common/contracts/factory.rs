use anyhow::Result;
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_multi_test::ContractWrapper;
use sg2::msg::{CollectionParams, CreateMinterMsg};
use sg721::{CollectionInfo, InstantiateMsg as Sg721InstantiateMsg, RoyaltyInfoResponse};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::{
    AIRDROP_MINT_FEE_BPS, AIRDROP_MINT_PRICE, CREATION_FEE, MAX_PER_ADDRESS_LIMIT, MAX_TOKEN_LIMIT,
    MAX_TRADING_OFFSET_SECS, MINT_FEE_BPS, MIN_MINT_PRICE, MINT_PRICE, SHUFFLE_FEE,
};
use tiles::contract::msg::InstantiateMsg as TilesInstantiateMsg;
use vending_factory::{
    msg::{
        InstantiateMsg as FactoryInstantiateMsg,
        ExecuteMsg as FactoryExecuteMsg,
        VendingMinterInitMsgExtension,
    },
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
        println!("Creating minter with:");
        println!("  Sender: {}", sender);
        println!("  Factory address: {}", self.contract_addr);
        println!("  Collection params: {:#?}", collection_params);
        println!("  Init msg: {:#?}", init_msg);

        app.execute_contract(
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
        collection_code_id: u64,
    ) -> Result<(Addr, Addr), anyhow::Error> {
        let collection_info = CollectionInfo {
            creator: creator.to_string(),
            description: "Test Collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            explicit_content: None,
            start_trading_time: None,
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: creator.to_string(),
                share: Decimal::percent(5),
            }),
        };

        let collection_params = CollectionParams {
            code_id: collection_code_id,
            name: "Test Collection".to_string(),
            symbol: "TEST".to_string(),
            info: collection_info.clone(),
        };

        let block_time = app.inner().block_info().time;
        let init_msg = VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://test/".to_string(),
            payment_address: Some(creator.to_string()),
            start_time: block_time.plus_seconds(86400), // Start in 1 day
            num_tokens: 100,
            mint_price: Coin::new(MINT_PRICE, NATIVE_DENOM),
            per_address_limit: MAX_PER_ADDRESS_LIMIT,
            whitelist: None,
        };

        let res = self.create_minter(
            app,
            creator,
            collection_params,
            init_msg,
        )?;

        // Extract contract addresses
        let minter_addr = res
            .events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| {
                let code_id_attr = e.attributes.iter().find(|a| a.key == "code_id")?;
                if code_id_attr.value == "2" {
                    e.attributes.iter().find(|a| a.key == "_contract_addr")
                } else {
                    None
                }
            })
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("Minter address not found in events");

        let sg721_addr = res
            .events
            .iter()
            .find(|e| {
                e.ty == "wasm"
                    && e.attributes
                        .iter()
                        .any(|a| a.key == "action" && a.value == "instantiate_sg721_reply")
            })
            .and_then(|e| e.attributes.iter().find(|a| a.key == "sg721_address"))
            .map(|a| Addr::unchecked(a.value.clone()))
            .expect("SG721 address not found in events");

        Ok((minter_addr, sg721_addr))
    }
}
