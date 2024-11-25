#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;
pub mod errors;
mod ed25519;
declare_id!("5mMJAQEN4MAQ5aKB9ntrhQGhRKNPF98d6UcsaiNXnayn");

#[program]
pub mod spl_token_minter {
    use super::*;
    
    pub fn mint_token(
        ctx: Context<MintToken>,
        msg: Vec<u8>,
        sig: [u8; 64],
        amount: u64
    ) -> Result<()> {
        mint::mint_token(ctx, msg,sig,amount)
    }
}