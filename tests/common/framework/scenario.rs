use cosmwasm_std::Addr;

#[derive(Debug, Clone)]
pub struct TokenMint {
    pub owner: Addr,
    pub token_id: u32,
}

#[derive(Debug, Clone)]
pub struct PixelUpdate {
    pub token_id: u32,
    pub pixel_id: u32,
    pub color: String,
    pub owner: Addr,
}

pub struct Scenario {
    pub minted_tokens: Vec<TokenMint>,
    pub pixel_updates: Vec<PixelUpdate>,
}

impl Scenario {
    pub fn new() -> Self {
        Self {
            minted_tokens: Vec::new(),
            pixel_updates: Vec::new(),
        }
    }

    pub fn record_mint(&mut self, owner: Addr, token_id: u32) {
        self.minted_tokens.push(TokenMint { owner, token_id });
    }

    pub fn record_pixel_update(&mut self, owner: Addr, token_id: u32, pixel_id: u32, color: String) {
        self.pixel_updates.push(PixelUpdate {
            token_id,
            pixel_id,
            color,
            owner,
        });
    }

    pub fn get_token_owner(&self, token_id: u32) -> Option<&Addr> {
        self.minted_tokens
            .iter()
            .find(|t| t.token_id == token_id)
            .map(|t| &t.owner)
    }

    pub fn get_last_pixel_update(&self, token_id: u32, pixel_id: u32) -> Option<&PixelUpdate> {
        self.pixel_updates
            .iter()
            .rev()
            .find(|p| p.token_id == token_id && p.pixel_id == pixel_id)
    }
}

impl Default for Scenario {
    fn default() -> Self {
        Self::new()
    }
} 