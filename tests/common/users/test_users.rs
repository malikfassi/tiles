use super::roles::{UserConfig, UserRole};
use crate::common::app::TestApp;
use cosmwasm_std::{Addr, Coin};
use sg_std::NATIVE_DENOM;
use std::collections::HashMap;
use tiles::defaults::constants::MINT_PRICE;

#[derive(Debug)]
pub struct User {
    pub address: Addr,
    pub config: UserConfig,
}

impl User {
    pub fn new(name: &str, config: UserConfig) -> Self {
        Self {
            address: Addr::unchecked(name),
            config,
        }
    }

    pub fn fund_account(&self, app: &mut TestApp) {
        if self.config.initial_balance > 0 {
            app.inner_mut().init_modules(|router, _, storage| {
                router
                    .bank
                    .init_balance(
                        storage,
                        &self.address,
                        vec![Coin::new(self.config.initial_balance, NATIVE_DENOM)],
                    )
                    .unwrap();
            });
        }
    }
}

pub struct TestUsers {
    users: HashMap<UserRole, User>,
}

impl TestUsers {
    pub fn new() -> Self {
        let mut users = HashMap::new();

        // Add admin users
        users.insert(UserRole::Admin, User::new("admin111", UserConfig::admin()));

        // Add collection owner
        users.insert(UserRole::Owner, User::new("owner111", UserConfig::owner()));

        // Add buyers
        users.insert(UserRole::Buyer, User::new("buyer111", UserConfig::buyer()));
        users.insert(UserRole::Whale, User::new("whale111", UserConfig::whale()));
        users.insert(
            UserRole::Poor,
            User::new("poor111", UserConfig::poor(MINT_PRICE.try_into().unwrap())),
        );

        // Add operator
        users.insert(
            UserRole::Operator,
            User::new("operator111", UserConfig::operator()),
        );

        // Add tile contract creator
        users.insert(
            UserRole::TileContractCreator,
            User::new("creator111", UserConfig::tile_contract_creator()),
        );

        // Add factory contract creator
        users.insert(
            UserRole::FactoryContractCreator,
            User::new("factory111", UserConfig::factory_contract_creator()),
        );

        Self { users }
    }

    pub fn get(&self, role: UserRole) -> &User {
        self.users.get(&role).expect("User role not found")
    }

    pub fn fund_all(&self, app: &mut TestApp) {
        for user in self.users.values() {
            user.fund_account(app);
        }
    }

    pub fn assert_balance(&self, app: &TestApp, role: UserRole, expected_balance: u128) {
        let balance = app
            .get_balance(&self.get(role).address, "ustars")
            .unwrap_or(0);
        assert_eq!(balance, expected_balance);
    }

    pub fn get_buyer(&self) -> &User {
        self.get(UserRole::Buyer)
    }

    pub fn user1(&self) -> Addr {
        self.get(UserRole::Buyer).address.clone()
    }

    pub fn poor_user(&self) -> &User {
        self.get(UserRole::Poor)
    }

    pub fn tile_contract_creator(&self) -> Addr {
        self.get(UserRole::TileContractCreator).address.clone()
    }

    pub fn factory_contract_creator(&self) -> Addr {
        self.get(UserRole::FactoryContractCreator).address.clone()
    }

    pub fn admin(&self) -> Addr {
        self.get(UserRole::Admin).address.clone()
    }

    pub fn fund_all_accounts(&self, app: &mut TestApp) {
        for user in self.users.values() {
            user.fund_account(app);
        }
    }
}

impl Default for TestUsers {
    fn default() -> Self {
        Self::new()
    }
}
