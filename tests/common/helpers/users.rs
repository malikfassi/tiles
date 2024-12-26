use crate::common::test_module::TilesApp;
use cosmwasm_std::{Addr, Coin};
use sg_std::NATIVE_DENOM;
use tiles::defaults::constants::MINT_PRICE;

pub struct TestUser {
    pub address: Addr,
    pub is_admin: bool,
    pub initial_balance: u128,
}

impl TestUser {
    pub fn new(name: &str, is_admin: bool, initial_balance: u128) -> Self {
        Self {
            address: Addr::unchecked(name),
            is_admin,
            initial_balance,
        }
    }

    pub fn admin(name: &str) -> Self {
        Self::new(name, true, 10_000_000_000)
    }

    pub fn regular(name: &str) -> Self {
        Self::new(name, false, 1_000_000_000)
    }

    pub fn poor(name: &str) -> Self {
        Self::new(name, false, MINT_PRICE / 2)
    }

    pub fn contract(name: &str) -> Self {
        Self::new(name, false, 0) // Contract addresses don't need funds
    }

    pub fn fund_account(&self, app: &mut TilesApp) {
        if self.initial_balance > 0 {
            app.init_modules(|router, _, storage| {
                router
                    .bank
                    .init_balance(
                        storage,
                        &self.address,
                        vec![Coin::new(self.initial_balance, NATIVE_DENOM)],
                    )
                    .unwrap();
            });
        }
    }
}

pub struct TestUsers {
    // Contract admins
    pub contract_admin: TestUser, // Admin of the tiles contract
    pub factory_admin: TestUser,  // Admin of the vending factory
    pub minter_admin: TestUser,   // Admin of the vending minter

    // Collection roles
    pub collection_owner: TestUser, // Owner of the NFT collection
    pub royalty_receiver: TestUser, // Receives royalties from sales
    pub minter: TestUser,           // Has permission to mint NFTs

    // Buyers with different balances
    pub whale_buyer: TestUser,   // Has lots of funds
    pub regular_buyer: TestUser, // Has enough funds for normal operations
    pub poor_buyer: TestUser,    // Has insufficient funds

    // Special roles
    pub operator: TestUser, // Has special permissions (like updating metadata)
    pub random_user: TestUser, // Just a random user with no special permissions
}

impl Default for TestUsers {
    fn default() -> Self {
        Self {
            // Contract admins
            contract_admin: TestUser::admin("contract_admin"),
            factory_admin: TestUser::admin("factory_admin"),
            minter_admin: TestUser::admin("minter_admin"),

            // Collection roles
            collection_owner: TestUser::regular("collection_owner"),
            royalty_receiver: TestUser::regular("royalty_receiver"),
            minter: TestUser::regular("minter"),

            // Buyers
            whale_buyer: TestUser::new("whale", false, 100_000_000_000),
            regular_buyer: TestUser::regular("buyer"),
            poor_buyer: TestUser::poor("poor_buyer"),

            // Special roles
            operator: TestUser::regular("operator"),
            random_user: TestUser::regular("random"),
        }
    }
}

impl TestUsers {
    pub fn fund_all(&self, app: &mut TilesApp) {
        // Fund admins
        self.contract_admin.fund_account(app);
        self.factory_admin.fund_account(app);
        self.minter_admin.fund_account(app);

        // Fund collection roles
        self.collection_owner.fund_account(app);
        self.royalty_receiver.fund_account(app);
        self.minter.fund_account(app);

        // Fund buyers
        self.whale_buyer.fund_account(app);
        self.regular_buyer.fund_account(app);
        self.poor_buyer.fund_account(app);

        // Fund special roles
        self.operator.fund_account(app);
        self.random_user.fund_account(app);
    }
}
