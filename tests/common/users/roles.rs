#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserRole {
    Admin,
    Owner,
    Buyer,
    Poor,
    Whale,
    Operator,
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
            initial_balance: 1_000_000_000_000,
        }
    }

    pub fn owner() -> Self {
        Self {
            role: UserRole::Owner,
            initial_balance: 1_000_000_000_000,
        }
    }

    pub fn buyer() -> Self {
        Self {
            role: UserRole::Buyer,
            initial_balance: 10_000_000_000,
        }
    }

    pub fn whale() -> Self {
        Self {
            role: UserRole::Whale,
            initial_balance: 10_000_000_000_000,
        }
    }

    pub fn poor(mint_price: u128) -> Self {
        Self {
            role: UserRole::Poor,
            initial_balance: mint_price / 2,
        }
    }

    pub fn operator() -> Self {
        Self {
            role: UserRole::Operator,
            initial_balance: 1_000_000_000_000,
        }
    }
} 