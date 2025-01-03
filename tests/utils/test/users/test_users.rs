use cosmwasm_std::{Addr, Coin, Uint128};
use sg_std::NATIVE_DENOM;

use crate::utils::core::app::TestApp;
use tiles::defaults::constants::{CREATION_FEE, MINT_PRICE};

#[derive(Clone)]
pub struct User {
    pub address: Addr,
}

impl User {
    pub fn new(address: &str) -> Self {
        Self {
            address: Addr::unchecked(address),
        }
    }
}

#[derive(Clone)]
pub struct TestUsers {
    pub buyer: User,
    pub tile_creator: User,
    pub factory_creator: User,
    pub poor_user: User,
    pub pixel_operator: User,
    pub creator: User,
}

impl Default for TestUsers {
    fn default() -> Self {
        Self::new()
    }
}

impl TestUsers {
    pub fn new() -> Self {
        Self {
            buyer: User::new("buyer"),
            tile_creator: User::new("tile_creator"),
            factory_creator: User::new("factory_creator"),
            poor_user: User::new("poor_user"),
            pixel_operator: User::new("pixel_operator"),
            creator: User::new("creator"),
        }
    }

    pub fn init_balances(&self, app: &mut TestApp) {
        // Fund all users except poor_user with enough for multiple operations
        let users_to_fund = [
            &self.buyer,
            &self.tile_creator,
            &self.factory_creator,
            &self.pixel_operator,
            &self.creator,
        ];

        for user in users_to_fund {
            app.inner_mut().init_modules(|router, _, storage| {
                router
                    .bank
                    .init_balance(
                        storage,
                        &user.address,
                        vec![Coin {
                            denom: NATIVE_DENOM.to_string(),
                            amount: Uint128::from((CREATION_FEE + MINT_PRICE) * 10u128),
                        }],
                    )
                    .unwrap();
            });
        }

        // Poor user gets minimal balance
        app.inner_mut().init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &self.poor_user.address,
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::from(MINT_PRICE / 2u128),
                    }],
                )
                .unwrap();
        });
    }

    pub fn get_buyer(&self) -> &User {
        &self.buyer
    }

    pub fn tile_contract_creator(&self) -> &User {
        &self.tile_creator
    }

    pub fn factory_contract_creator(&self) -> &User {
        &self.factory_creator
    }

    pub fn poor_user(&self) -> &User {
        &self.poor_user
    }

    pub fn pixel_operator(&self) -> &User {
        &self.pixel_operator
    }

    pub fn creator(&self) -> &User {
        &self.creator
    }
}
