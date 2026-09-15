use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub min_entry_fee: u64,
}
