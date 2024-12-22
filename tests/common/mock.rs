use cosmwasm_std::testing::mock_dependencies;
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper};

pub fn mock_app() -> App {
    AppBuilder::new().build(|_router, _, _storage| {})
}

pub fn contract_vending_factory() -> Box<dyn Contract> {
    let contract = ContractWrapper::new(
        vending_factory::contract::execute,
        vending_factory::contract::instantiate,
        vending_factory::contract::query,
    )
    .with_sudo(vending_factory::contract::sudo);
    Box::new(contract)
}

pub fn contract_vending_minter() -> Box<dyn Contract> {
    let contract = ContractWrapper::new(
        vending_minter::contract::execute,
        vending_minter::contract::instantiate,
        vending_minter::contract::query,
    );
    Box::new(contract)
}

pub fn contract_sg721() -> Box<dyn Contract> {
    let contract = ContractWrapper::new(
        sg721_base::entry::execute,
        sg721_base::entry::instantiate,
        sg721_base::entry::query,
    );
    Box::new(contract)
} 