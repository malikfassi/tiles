use cosmwasm_std::Addr;

// Test addresses
pub const ADMIN: &str = "admin";
pub const MINTER: &str = "minter";
pub const CREATOR: &str = "creator";
pub const DEVELOPER: &str = "developer";
pub const USER1: &str = "user1";
pub const USER2: &str = "user2";
pub const UNAUTHORIZED: &str = "unauthorized";

// Contract addresses (for tests)
pub const FACTORY_CONTRACT: &str = "contract0";
pub const MINTER_CONTRACT: &str = "contract1";
pub const COLLECTION_CONTRACT: &str = "contract2";

pub struct TestAddresses {
    pub admin: Addr,
    pub minter: Addr,
    pub creator: Addr,
    pub developer: Addr,
    pub user1: Addr,
    pub user2: Addr,
    pub unauthorized: Addr,
    pub factory: Addr,
    pub minter_contract: Addr,
    pub collection: Addr,
}

impl Default for TestAddresses {
    fn default() -> Self {
        Self {
            admin: Addr::unchecked(ADMIN),
            minter: Addr::unchecked(MINTER),
            creator: Addr::unchecked(CREATOR),
            developer: Addr::unchecked(DEVELOPER),
            user1: Addr::unchecked(USER1),
            user2: Addr::unchecked(USER2),
            unauthorized: Addr::unchecked(UNAUTHORIZED),
            factory: Addr::unchecked(FACTORY_CONTRACT),
            minter_contract: Addr::unchecked(MINTER_CONTRACT),
            collection: Addr::unchecked(COLLECTION_CONTRACT),
        }
    }
}

// Helper functions for tests
pub fn mock_addresses() -> TestAddresses {
    TestAddresses::default()
}

pub fn mock_admin() -> Addr {
    Addr::unchecked(ADMIN)
}

pub fn mock_minter() -> Addr {
    Addr::unchecked(MINTER)
}

pub fn mock_creator() -> Addr {
    Addr::unchecked(CREATOR)
}

pub fn mock_developer() -> Addr {
    Addr::unchecked(DEVELOPER)
}

pub fn mock_user1() -> Addr {
    Addr::unchecked(USER1)
}

pub fn mock_user2() -> Addr {
    Addr::unchecked(USER2)
}

pub fn mock_unauthorized() -> Addr {
    Addr::unchecked(UNAUTHORIZED)
}
