use tiles::defaults::constants::{CREATION_FEE, MINT_PRICE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserRole {
    Admin,
    Owner,
    Buyer,
    Whale,
    Poor,
    Operator,
    TileContractCreator,
    FactoryContractCreator,
}

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub role: UserRole,
    pub initial_balance: u128,
}

impl UserConfig {
    pub fn admin() -> Self {
        Self {
            role: UserRole::Admin,
            initial_balance: 1_000_000_000,
        }
    }

    pub fn owner() -> Self {
        Self {
            role: UserRole::Owner,
            initial_balance: 1_000_000_000,
        }
    }

    pub fn buyer() -> Self {
        Self {
            role: UserRole::Buyer,
            initial_balance: MINT_PRICE * 10,
        }
    }

    pub fn whale() -> Self {
        Self {
            role: UserRole::Whale,
            initial_balance: MINT_PRICE * 100,
        }
    }

    pub fn poor(min_balance: u64) -> Self {
        Self {
            role: UserRole::Poor,
            initial_balance: min_balance as u128 / 2,
        }
    }

    pub fn operator() -> Self {
        Self {
            role: UserRole::Operator,
            initial_balance: 1_000_000_000,
        }
    }

    pub fn tile_contract_creator() -> Self {
        Self {
            role: UserRole::TileContractCreator,
            initial_balance: MINT_PRICE + CREATION_FEE + 10_000_000_000,
        }
    }

    pub fn factory_contract_creator() -> Self {
        Self {
            role: UserRole::FactoryContractCreator,
            initial_balance: MINT_PRICE + CREATION_FEE + 10_000_000_000,
        }
    }
}
