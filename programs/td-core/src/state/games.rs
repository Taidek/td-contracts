pub use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Game {
    #[max_len(20)]
    name: String,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Server {
    #[max_len(20)]
    name: String,
}
