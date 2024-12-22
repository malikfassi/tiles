use sg721_base::msg::CollectionInfoResponse;
use crate::types::Config;

pub fn default_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg {
        name: "Tiles".to_string(),
        symbol: "TILE".to_string(),
        minter: DEFAULT_MINTER.to_string(),
        collection_info: CollectionInfoResponse {
            creator: DEFAULT_ADMIN.to_string(),
            description: "A collection of tiles".to_string(),
            image: "ipfs://image".to_string(),
            external_link: None,
            explicit_content: None,
            start_trading_time: None,
            royalty_info: None,
        },
        config: Config::default(),
    }
} 