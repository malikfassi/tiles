use crate::common::contracts::tiles::TilesContract;
use cosmwasm_std::{Addr, Coin, StdError, StdResult};
use cw_multi_test::{ContractWrapper, Executor};
use sg2::msg::{CreateMinterMsg, Sg2ExecuteMsg};
use sg_multi_test::StargazeApp as App;
use sg_std::NATIVE_DENOM;
use vending_factory::contract::{execute, instantiate, query, sudo};
use vending_factory::msg::InstantiateMsg;
use vending_factory::state::VendingMinterParams;

pub struct VendingFactory {
    pub addr: Addr,
}

impl VendingFactory {
    pub fn new(app: &mut App, admin: &Addr, minter_code_id: u64, collection_code_id: u64) -> Self {
        println!("\n=== Creating new VendingFactory ===");
        // Store contract code
        let code_id = store_vending_factory_code(app).unwrap();
        println!("✓ Stored factory code with ID: {}", code_id);

        // Instantiate contract with default params
        println!("Instantiating factory with default params...");
        let msg = InstantiateMsg {
            params: VendingMinterParams {
                code_id: minter_code_id,
                allowed_sg721_code_ids: vec![collection_code_id],
                frozen: false,
                creation_fee: Coin::new(1000000, NATIVE_DENOM),
                min_mint_price: Coin::new(100000, NATIVE_DENOM),
                mint_fee_bps: 1000,
                max_trading_offset_secs: 60 * 60 * 24 * 7, // 1 week
                extension: vending_factory::state::ParamsExtension {
                    max_token_limit: 1000,
                    max_per_address_limit: 50,
                    airdrop_mint_price: Coin::new(0, NATIVE_DENOM),
                    airdrop_mint_fee_bps: 0,
                    shuffle_fee: Coin::new(0, NATIVE_DENOM),
                },
            },
        };

        println!("Factory params:");
        println!("- Code ID: {}", minter_code_id);
        println!("- Allowed SG721 code IDs: [{}]", collection_code_id);
        println!("- Creation fee: 1000000 {}", NATIVE_DENOM);
        println!("- Min mint price: 100000 {}", NATIVE_DENOM);
        println!("- Mint fee bps: 1000");
        println!("- Max trading offset: 1 week");

        let addr = app
            .instantiate_contract(
                code_id,
                admin.clone(),
                &msg,
                &[],
                "Test Vending Factory",
                None,
            )
            .unwrap();

        println!("✓ Factory instantiated at address: {}", addr);
        println!("=== VendingFactory creation complete ===\n");

        Self { addr }
    }

    pub fn create_minter(
        &self,
        app: &mut App,
        sender: &Addr,
        msg: CreateMinterMsg<vending_factory::msg::VendingMinterInitMsgExtension>,
    ) -> StdResult<(Addr, TilesContract)> {
        println!("\n=== Creating minter through factory ===");
        println!("Factory address: {}", self.addr);
        println!("Sender: {}", sender);
        println!("Creation fee: 1000000 {}", NATIVE_DENOM);
        
        // First, execute the create_minter message on the factory
        println!("Executing create_minter message...");
        let res = match app.execute_contract(
            sender.clone(),
            self.addr.clone(),
            &Sg2ExecuteMsg::CreateMinter(msg.clone()),
            &[Coin::new(1000000, NATIVE_DENOM)],
        ) {
            Ok(res) => {
                println!("✓ Create minter message executed successfully");
                res
            },
            Err(e) => {
                println!("❌ Failed to execute create_minter message");
                println!("Error: {:#?}", e);
                return Err(StdError::generic_err(format!("Failed to create minter: {}", e)));
            }
        };

        println!("\nParsing response events...");
        // Get the minter address from the instantiate event
        let minter_addr = match res.events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| e.attributes.iter().find(|a| a.key == "_contract_address"))
            .map(|a| Addr::unchecked(a.value.clone()))
        {
            Some(addr) => {
                println!("✓ Found minter address: {}", addr);
                addr
            },
            None => {
                println!("❌ Could not find minter address in response");
                return Err(StdError::generic_err("Could not find minter address in response"));
            }
        };

        // Get the collection address from the instantiate event
        let collection_addr = match res.events
            .iter()
            .find(|e| e.ty == "instantiate")
            .and_then(|e| e.attributes.iter().find(|a| a.key == "_contract_address"))
            .map(|a| Addr::unchecked(a.value.clone()))
        {
            Some(addr) => {
                println!("✓ Found collection address: {}", addr);
                addr
            },
            None => {
                println!("❌ Could not find collection address in response");
                return Err(StdError::generic_err("Could not find collection address in response"));
            }
        };

        println!("=== Minter creation complete ===\n");
        Ok((minter_addr, TilesContract::new(collection_addr)))
    }
}

pub fn store_vending_factory_code(app: &mut App) -> StdResult<u64> {
    println!("Creating vending factory contract wrapper...");
    let contract = ContractWrapper::new(execute, instantiate, query)
        .with_sudo(sudo);
    println!("Storing vending factory code...");
    let code_id = app.store_code(Box::new(contract));
    println!("✓ Vending factory code stored successfully");
    Ok(code_id)
}
