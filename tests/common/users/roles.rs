use cosmwasm_std::Addr;

#[derive(Clone)]
pub struct User {
    pub address: Addr,
    pub role: UserRole,
}

#[derive(Clone, Copy, Debug)]
pub enum UserRole {
    FactoryCreator,
    TileCreator,
    Buyer,
    PoorUser,
    PixelOperator,
    Creator,
}

impl User {
    pub fn new(address: &str, role: UserRole) -> Self {
        Self {
            address: Addr::unchecked(address),
            role,
        }
    }
}
