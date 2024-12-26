use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, CustomQuery, Empty, Querier, Storage,
    testing::{MockApi, MockQuerier, MockStorage},
};
use cw_multi_test::{
    AppResponse, CosmosRouter, Module,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use sg_std::{StargazeMsgWrapper, StargazeQuery};
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct TilesModule {}

impl Module for TilesModule {
    type ExecT = StargazeMsgWrapper;
    type QueryT = StargazeQuery;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: StargazeMsgWrapper,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        // Delegate to the parent module
        let stargaze_module = sg_multi_test::StargazeModule {};
        stargaze_module.execute(api, storage, router, block, sender, msg)
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!("sudo not implemented for TilesModule")
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        msg: StargazeQuery,
    ) -> AnyResult<Binary> {
        bail!("query not implemented for TilesModule: {:?}", msg)
    }
}

pub type TilesApp = sg_multi_test::StargazeApp;

pub type TilesDeps = cosmwasm_std::OwnedDeps<MockStorage, MockApi, MockQuerier, StargazeQuery>;

pub fn mock_tiles_deps() -> TilesDeps {
    cosmwasm_std::OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
} 