use super::roles::{UserRole, UserConfig};
use crate::common::app::TestApp;
use cosmwasm_std::{Addr, Coin};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::MINT_PRICE;
use std::collections::HashMap;

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
        users.insert(UserRole::Admin, User::new("admin", UserConfig::admin()));
        
        // Add collection owner
        users.insert(UserRole::Owner, User::new("owner", UserConfig::owner()));
        
        // Add buyers
        users.insert(UserRole::Buyer, User::new("buyer", UserConfig::buyer()));
        users.insert(UserRole::Whale, User::new("whale", UserConfig::whale()));
        users.insert(UserRole::Poor, User::new("poor", UserConfig::poor(MINT_PRICE)));
        
        // Add operator
        users.insert(UserRole::Operator, User::new("operator", UserConfig::operator()));

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
        let user = self.get(role);
        let balance = app
            .inner()
            .wrap()
            .query_balance(&user.address, NATIVE_DENOM)
            .unwrap();
        assert_eq!(balance.amount.u128(), expected_balance);
    }
}

impl Default for TestUsers {
    fn default() -> Self {
        Self::new()
    }
} 