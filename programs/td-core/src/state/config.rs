pub use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct CoreConfig {
    pub bump: u8,
    pub authorization: Pubkey,
}
