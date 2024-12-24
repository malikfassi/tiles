#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Empty,
    };
    use sg721_base::msg::ExecuteMsg as Sg721ExecuteMsg;

    use crate::contract::{
        contract::{execute, instantiate, query},
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
        state::{Config, PriceScaling},
    };

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            minter: "minter".to_string(),
            dev_address: "dev".to_string(),
            dev_fee_percent: 5,
            base_price: 100_000_000,
            price_scaling: PriceScaling {
                hour_1_price: 100_000_000,
                hour_12_price: 200_000_000,
                hour_24_price: 300_000_000,
                quadratic_base: 400_000_000,
            },
        };

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Query config
        let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
        let config: Config = from_binary(&res).unwrap();
        assert_eq!(config.admin, Addr::unchecked("creator"));
        assert_eq!(config.minter, Addr::unchecked("minter"));
        assert_eq!(config.dev_address, Addr::unchecked("dev"));
        assert_eq!(config.dev_fee_percent, 5);
        assert_eq!(config.base_price, 100_000_000);
    }

    #[test]
    fn update_config() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Instantiate
        let msg = InstantiateMsg {
            minter: "minter".to_string(),
            dev_address: "dev".to_string(),
            dev_fee_percent: 5,
            base_price: 100_000_000,
            price_scaling: PriceScaling {
                hour_1_price: 100_000_000,
                hour_12_price: 200_000_000,
                hour_24_price: 300_000_000,
                quadratic_base: 400_000_000,
            },
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Update config
        let msg = ExecuteMsg::UpdateConfig {
            dev_address: Some("new_dev".to_string()),
            dev_fee_percent: Some(10),
            base_price: Some(200_000_000),
            price_scaling: Some(PriceScaling {
                hour_1_price: 200_000_000,
                hour_12_price: 300_000_000,
                hour_24_price: 400_000_000,
                quadratic_base: 500_000_000,
            }),
        };
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query updated config
        let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
        let config: Config = from_binary(&res).unwrap();
        assert_eq!(config.dev_address, Addr::unchecked("new_dev"));
        assert_eq!(config.dev_fee_percent, 10);
        assert_eq!(config.base_price, 200_000_000);
    }

    #[test]
    fn set_pixel_color() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(100_000_000, "ustars"));

        // Instantiate
        let msg = InstantiateMsg {
            minter: "minter".to_string(),
            dev_address: "dev".to_string(),
            dev_fee_percent: 5,
            base_price: 100_000_000,
            price_scaling: PriceScaling {
                hour_1_price: 100_000_000,
                hour_12_price: 200_000_000,
                hour_24_price: 300_000_000,
                quadratic_base: 400_000_000,
            },
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Mint token
        let mint_msg = ExecuteMsg::Sg721(Sg721ExecuteMsg::Mint {
            token_id: "1".to_string(),
            owner: "creator".to_string(),
            token_uri: None,
            extension: Empty {},
        });
        let mint_info = mock_info("minter", &[]);
        execute(deps.as_mut(), env.clone(), mint_info, mint_msg).unwrap();

        // Set pixel color
        let msg = ExecuteMsg::SetPixelColor {
            token_id: "1".to_string(),
            color: "#FF0000".to_string(),
            position: 0,
            expiration: 3600,
        };
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query pixel state
        let res = query(
            deps.as_ref(),
            env,
            QueryMsg::PixelState {
                token_id: "1".to_string(),
                position: 0,
            },
        )
        .unwrap();
        let pixel: crate::contract::msg::PixelStateResponse = from_binary(&res).unwrap();
        assert_eq!(pixel.color, "#FF0000");
        assert_eq!(pixel.position, 0);
        assert!(pixel.expiration > 0);
    }
} 