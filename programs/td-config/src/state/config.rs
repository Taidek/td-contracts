pub use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct GlobalConfig {
    pub authorization: Pubkey,
    pub oracle: Pubkey,
    pub token_mint: Pubkey,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct PayerRecord {
    bump: u8,
}
