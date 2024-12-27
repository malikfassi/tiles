use tiles::defaults::constants::{CREATION_FEE, MINT_PRICE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserRole {
    Buyer,                  // Person who buys a tile
    Poor,                   // User with insufficient funds (for negative testing)
    PixelOperator,          // User who can operate on pixels but isn't creator/buyer
    TileContractCreator,    // Creator of the tile contract, receives royalties
    FactoryContractCreator, // Creator of the factory contract
}

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub role: UserRole,
    pub initial_balance: u128,
}

impl UserConfig {
    pub fn buyer() -> Self {
        Self {
            role: UserRole::Buyer,
            initial_balance: MINT_PRICE * 10,
        }
    }

    pub fn poor(min_balance: u64) -> Self {
        Self {
            role: UserRole::Poor,
            initial_balance: min_balance as u128 / 2,
        }
    }

    pub fn pixel_operator() -> Self {
        Self {
            role: UserRole::PixelOperator,
            initial_balance: MINT_PRICE * 5, // Enough for some operations but less than buyer
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
