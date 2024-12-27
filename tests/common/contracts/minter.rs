use anyhow::Result;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::ContractWrapper;
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::MINT_PRICE;
use vending_minter::msg::{ConfigResponse, ExecuteMsg as MinterExecuteMsg, QueryMsg};

use crate::common::TestApp;

pub struct MinterContract {
    pub contract_addr: Addr,
}

impl MinterContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    pub fn store_code(app: &mut TestApp) -> Result<u64> {
        let contract = Box::new(
            ContractWrapper::new(
                vending_minter::contract::execute,
                vending_minter::contract::instantiate,
                vending_minter::contract::query,
            )
            .with_reply(vending_minter::contract::reply),
        );
        Ok(app.store_code(contract))
    }

    pub fn mint(&self, app: &mut TestApp, user: &Addr) -> Result<u32> {
        println!("\n=== Starting mint process ===");
        println!("Minting token:");
        println!("  User: {}", user);
        println!("  Minter address: {}", self.contract_addr);
        println!("  Mint price: {}", MINT_PRICE);

        // Query minter config to verify setup
        let config: ConfigResponse = app
            .inner()
            .wrap()
            .query_wasm_smart(self.contract_addr.clone(), &QueryMsg::Config {})?;
        println!("Minter config: {:#?}", config);

        // Mint through the minter contract
        let response = app.execute_contract(
            user.clone(),
            self.contract_addr.clone(),
            &MinterExecuteMsg::Mint {},
            &[Coin::new(MINT_PRICE, NATIVE_DENOM)],
        );

        match &response {
            Ok(res) => {
                println!("Mint successful!");
                println!("Response events: {:#?}", res.events);
            }
            Err(e) => {
                println!("Mint failed!");
                println!("Error: {:#?}", e);
            }
        }

        let response = response?;

        // Extract token_id from response
        let token_id = response
            .events
            .iter()
            .find(|e| e.ty == "wasm")
            .and_then(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "token_id")
                    .map(|a| a.value.parse::<u32>().unwrap())
            })
            .expect("Token ID not found in mint response");

        println!("Successfully minted token_id: {}", token_id);
        println!("=== Mint process complete ===\n");
        Ok(token_id)
    }
}
