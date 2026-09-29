use anyhow::Result;
use cosmwasm_std::{coins, Addr, Coin};
use cw721::msg::{NftInfoResponse, OwnerOfResponse};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};
use tiles::{
    contract::msg::{ExecuteMsg, QueryMsg, TileExecuteMsg, TileQueryMsg},
    core::{
        pricing::PriceScaling,
        tile::{
            metadata::{PixelUpdate, TileMetadata},
            Tile,
        },
    },
    defaults::constants::NATIVE_DENOM,
};

use crate::utils::core::app::TestApp;

pub struct TilesContract {
    pub contract_addr: Addr,
}

impl TilesContract {
    pub fn new(contract_addr: Addr) -> Self {
        Self { contract_addr }
    }

    pub fn store_code(app: &mut TestApp) -> Result<u64> {
        let contract = ContractWrapper::new(
            tiles::contract::execute,
            tiles::contract::instantiate,
            tiles::contract::query,
        );
        Ok(app.store_code(Box::new(contract)))
    }

    /// Mints a tile to `owner`.
    ///
    /// The collection's minter is the contract itself, so that minting can later be
    /// forced through the contract's own logic rather than an external minter contract.
    pub fn mint(&self, app: &mut TestApp, owner: &Addr, token_id: &str) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            self.contract_addr.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Mint {
                token_id: token_id.to_string(),
                owner: owner.to_string(),
                token_uri: None,
                extension: Tile {
                    tile_hash: TileMetadata::default().hash(),
                    metadata: TileMetadata::default(),
                },
            },
            &[],
        )
    }

    /// Sends a `Mint` from `sender` instead of from the contract.
    ///
    /// Used to check that the CW721 base refuses anyone but the configured minter.
    pub fn mint_as_stranger(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: &str,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Mint {
                token_id: token_id.to_string(),
                owner: sender.to_string(),
                token_uri: None,
                extension: Tile {
                    tile_hash: TileMetadata::default().hash(),
                    metadata: TileMetadata::default(),
                },
            },
            &[],
        )
    }

    /// Colours pixels, paying exactly the advertised price in the native denom.
    pub fn update_pixel(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
        current_metadata: TileMetadata,
    ) -> Result<AppResponse> {
        let price_scaling = self.query_price_scaling(app)?;
        let total_price = updates.iter().fold(0u128, |acc, update| {
            acc + price_scaling
                .calculate_price(update.expiration_duration)
                .u128()
        });

        self.update_pixel_with_funds(
            app,
            sender,
            token_id,
            updates,
            current_metadata,
            coins(total_price, NATIVE_DENOM),
        )
    }

    /// Colours pixels with an explicit amount, to test payment failures.
    pub fn update_pixel_with_funds(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: u32,
        updates: Vec<PixelUpdate>,
        current_metadata: TileMetadata,
        funds: Vec<Coin>,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::UpdateExtension {
                msg: TileExecuteMsg::SetPixelColor {
                    token_id: token_id.to_string(),
                    current_metadata,
                    updates,
                },
            },
            &funds,
        )
    }

    pub fn execute_update_price_scaling(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        new_price_scaling: PriceScaling,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::UpdateExtension {
                msg: TileExecuteMsg::UpdatePriceScaling(Box::new(new_price_scaling)),
            },
            &[],
        )
    }

    pub fn query_price_scaling(&self, app: &TestApp) -> Result<PriceScaling> {
        let value = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Extension {
                msg: TileQueryMsg::PriceScaling {},
            },
        )?;
        Ok(value)
    }

    /// Reads the payment split and price floor configured at instantiation.
    pub fn query_config(&self, app: &TestApp) -> Result<tiles::contract::state::Config> {
        let value = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Extension {
                msg: TileQueryMsg::Config {},
            },
        )?;
        Ok(value)
    }

    /// Reads the pixels of a tile through the dedicated query.
    pub fn query_tile_pixels(&self, app: &TestApp, token_id: u32) -> Result<TileMetadata> {
        let value = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::Extension {
                msg: TileQueryMsg::TilePixels {
                    token_id: token_id.to_string(),
                },
            },
        )?;
        Ok(value)
    }

    pub fn query_token_hash(&self, app: &TestApp, token_id: u32) -> Result<String> {
        let response: NftInfoResponse<Tile> = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::NftInfo {
                token_id: token_id.to_string(),
            },
        )?;
        Ok(response.extension.tile_hash)
    }

    pub fn query_owner(&self, app: &TestApp, token_id: u32) -> Result<String> {
        let response: OwnerOfResponse = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: None,
            },
        )?;
        Ok(response.owner)
    }

    /// String-typed owner lookup, for tests that carry the token id as text.
    pub fn query_owner_of(&self, app: &TestApp, token_id: String) -> Result<String> {
        let response: OwnerOfResponse = app.inner().wrap().query_wasm_smart(
            self.contract_addr.clone(),
            &QueryMsg::OwnerOf {
                token_id,
                include_expired: None,
            },
        )?;
        Ok(response.owner)
    }

    pub fn execute_transfer_nft(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        recipient: &Addr,
        token_id: String,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::TransferNft {
                recipient: recipient.to_string(),
                token_id,
            },
            &[],
        )
    }

    pub fn execute_approve(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        spender: &Addr,
        token_id: String,
        expires: Option<cw721::Expiration>,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Approve {
                spender: spender.to_string(),
                token_id,
                expires,
            },
            &[],
        )
    }

    pub fn execute_burn(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        token_id: String,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::Burn { token_id },
            &[],
        )
    }

    pub fn execute_send_nft(
        &self,
        app: &mut TestApp,
        sender: &Addr,
        contract: &Addr,
        token_id: String,
        msg: cosmwasm_std::Binary,
    ) -> Result<AppResponse> {
        app.inner_mut().execute_contract(
            sender.clone(),
            self.contract_addr.clone(),
            &ExecuteMsg::SendNft {
                contract: contract.to_string(),
                token_id,
                msg,
            },
            &[],
        )
    }

    pub fn assert_token_owner(&self, app: &TestApp, token_id: u32, expected: &Addr) {
        let owner = self
            .query_owner(app, token_id)
            .expect("owner query succeeds");
        assert_eq!(owner, expected.to_string(), "unexpected token owner");
    }
}
