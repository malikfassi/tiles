use sg721::{CollectionInfo, RoyaltyInfoResponse};

use crate::msg::InstantiateMsg;

pub fn default_collection_info(creator: String) -> CollectionInfo<RoyaltyInfoResponse> {
    CollectionInfo {
        creator,
        description: "A collection of customizable pixel tiles".to_string(),
        image: "ipfs://bafkreihdxc7zcyxykx7xskopr4yfk5lsv4bih4j4euqj6xv3s4u6wqpjc4".to_string(),
        external_link: None,
        explicit_content: None,
        start_trading_time: None,
        royalty_info: None,
    }
}

pub fn default_instantiate_msg(
    name: String,
    symbol: String,
    minter: String,
    creator: String,
) -> InstantiateMsg {
    InstantiateMsg {
        name,
        symbol,
        minter,
        collection_info: default_collection_info(creator),
    }
} 