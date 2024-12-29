pub mod app;
pub mod contracts;
pub mod event_assertions;
pub mod launchpad;
pub mod test_context;
pub mod users;

pub use event_assertions::EventAssertions;
pub use test_context::TestContext;
use cw_multi_test::AppResponse;

pub fn extract_token_id(response: &AppResponse) -> u32 {
    response
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .and_then(|e| {
            e.attributes
                .iter()
                .find(|a| a.key == "token_id")
                .map(|a| a.value.parse::<u32>().unwrap())
        })
        .expect("Token ID not found in mint response")
}
