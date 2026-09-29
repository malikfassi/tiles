/// Deploys the tiles collection and seeds test balances.
///
/// There is no vending factory/minter any more: the collection is instantiated
/// directly, and minting is done by the contract minter (the contract itself in
/// tests) through the standard CW721 `Mint` message.
use anyhow::Result;
use cosmwasm_std::{coins, Addr, Decimal};
use cw721::msg::{CollectionExtensionMsg, RoyaltyInfoResponse};
use cw721::Action;
use cw_multi_test::{AppResponse, Executor};

use tiles::{
    contract::instantiate::InstantiateMsg,
    contract::msg::ExecuteMsg,
    defaults::constants::{COLLECTION_DESCRIPTION, COLLECTION_NAME, COLLECTION_SYMBOL},
};

use crate::utils::{
    contracts::tiles::TilesContract,
    core::app::{TestApp, TEST_DENOM},
    TestUsers,
};

/// Initial balance given to every test user, in micro ATOM.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000;

/// Royalty share declared in the collection extension, as Stargaze 2.0 documents it.
pub const ROYALTY_SHARE: Decimal = Decimal::percent(5);

pub struct Launchpad {
    pub app: TestApp,
    pub users: TestUsers,
    pub tiles: TilesContract,
    pub code_id: u64,
}

impl Launchpad {
    /// Deploys the collection and returns a ready-to-use context.
    pub fn setup() -> Result<(Self, AppResponse)> {
        let mut app = TestApp::new();
        let users = TestUsers::with_app_addresses(&mut app);
        users.init_balances(&mut app);

        let code_id = TilesContract::store_code(&mut app)?;

        let creator = users.get_buyer().address.clone();
        let collection_info: CollectionExtensionMsg<RoyaltyInfoResponse> = CollectionExtensionMsg {
            description: Some(COLLECTION_DESCRIPTION.to_string()),
            // CW721 0.22 validates that the collection image is a well-formed URL.
            image: Some("ipfs://tiles-collection-image".to_string()),
            external_link: None,
            banner_url: None,
            explicit_content: None,
            start_trading_time: None,
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: creator.to_string(),
                share: ROYALTY_SHARE,
            }),
        };

        let msg = InstantiateMsg {
            name: COLLECTION_NAME.to_string(),
            symbol: COLLECTION_SYMBOL.to_string(),
            collection_info_extension: Some(collection_info),
            // The contract mints on its own behalf, so it must be trusted as minter.
            minter: None,
            creator: Some(addr_to_string(&creator)),
            withdraw_address: None,
        };

        let contract_addr = app.inner_mut().instantiate_contract(
            code_id,
            creator.clone(),
            &msg,
            &[],
            "tiles",
            None,
        )?;

        let tiles = TilesContract::new(contract_addr.clone());

        // The collection trusts its own address as minter: tiles are minted by the
        // contract, which seeds the 100 blank pixels before delegating to the CW721 base.
        app.inner_mut().execute_contract(
            creator.clone(),
            contract_addr.clone(),
            &ExecuteMsg::UpdateMinterOwnership(Action::TransferOwnership {
                new_owner: contract_addr.to_string(),
                expiry: None,
            }),
            &[],
        )?;
        app.inner_mut().execute_contract(
            contract_addr.clone(),
            contract_addr.clone(),
            &ExecuteMsg::UpdateMinterOwnership(Action::AcceptOwnership {}),
            &[],
        )?;

        Ok((
            Self {
                app,
                users,
                tiles,
                code_id,
            },
            AppResponse::default(),
        ))
    }

    /// Deploys the collection and mints one tile to the buyer.
    pub fn setup_with_token() -> Result<(Self, u32)> {
        let (mut launchpad, _) = Self::setup()?;
        let buyer = launchpad.users.get_buyer().address.clone();
        let _response = launchpad.mint(&buyer, "1")?;
        let token_id: u32 = "1".parse().expect("token id is numeric");
        Ok((launchpad, token_id))
    }

    /// Mints a tile with the given token id to `owner`.
    pub fn mint(&mut self, owner: &Addr, token_id: &str) -> Result<AppResponse> {
        self.tiles.mint(&mut self.app, owner, token_id)
    }
}

/// Test users all start with the same balance, in the chain's native denomination.
pub fn fund(app: &mut TestApp, address: &Addr) {
    app.inner_mut().init_modules(|router, _, storage| {
        router
            .bank
            .init_balance(storage, address, coins(INITIAL_BALANCE, TEST_DENOM))
            .expect("initial balance is set");
    });
}

fn addr_to_string(addr: &Addr) -> String {
    addr.to_string()
}
