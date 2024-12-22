use cosmwasm_std::{Addr, Coin, Timestamp};
use sg721::{CollectionInfo, RoyaltyInfoResponse};
use sg2::msg::{CollectionParams, CreateMinterMsg};
use vending_factory::msg::{InstantiateMsg, VendingMinterInitMsgExtension};
use vending_factory::state::{ParamsExtension, VendingMinterParams};

pub fn create_minter(
    app: &mut App,
    factory_code_id: u64,
    minter_code_id: u64,
    creator: &str,
) -> Result<Addr, anyhow::Error> {
    // First instantiate the factory
    let factory_addr = app.instantiate_contract(
        factory_code_id,
        Addr::unchecked(creator),
        &InstantiateMsg {
            params: VendingMinterParams {
                code_id: minter_code_id,
                creation_fee: Coin::new(1_000_000, "ustars"),
                min_mint_price: Coin::new(100_000_000, "ustars"),
                mint_fee_bps: 1000,
                max_trading_offset_secs: 604800, // 1 week
                extension: ParamsExtension {
                    max_token_limit: 10000,
                    max_per_address_limit: 50,
                    airdrop_mint_price: Coin::new(0, "ustars"),
                    airdrop_mint_fee_bps: 0,
                    shuffle_fee: Coin::new(1_000_000, "ustars"),
                },
                allowed_sg721_code_ids: vec![1],
                frozen: false,
            },
        },
        &[],
        "vending factory",
        Some(creator.to_string()),
    )?;

    // Then create a minter
    let create_minter_msg = CreateMinterMsg {
        init_msg: VendingMinterInitMsgExtension {
            base_token_uri: "ipfs://...".to_string(),
            payment_address: None,
            start_time: Timestamp::from_nanos(1_000_000_000),
            num_tokens: 100,
            mint_price: Coin::new(100_000_000, "ustars"),
            per_address_limit: 5,
            whitelist: None,
        },
        collection_params: CollectionParams {
            code_id: minter_code_id,
            name: "Test Collection".to_string(),
            symbol: "TEST".to_string(),
            info: CollectionInfo {
                creator: creator.to_string(),
                description: "Test description".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
        },
    };

    let res = app.execute_contract(
        Addr::unchecked(creator),
        factory_addr.clone(),
        &sg2::msg::Sg2ExecuteMsg::CreateMinter(create_minter_msg),
        &[Coin::new(1_000_000, "ustars")],
    )?;

    // Get the minter address from the response
    let minter_addr = res
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .and_then(|e| {
            e.attributes
                .iter()
                .find(|a| a.key == "minter")
                .map(|a| a.value.clone())
        })
        .ok_or_else(|| anyhow::anyhow!("Could not find minter address in response"))?;

    Ok(Addr::unchecked(minter_addr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{coins, Decimal};

    #[test]
    fn test_create_minter() {
        let mut deps = mock_dependencies();
        let mut app = App::new(deps.as_mut());

        let factory_code_id = app.store_code(contract_vending_factory());
        let minter_code_id = app.store_code(contract_vending_minter());

        let res = create_minter(&mut app, factory_code_id, minter_code_id, "creator");
        assert!(res.is_ok(), "Minter creation failed");
    }
} 