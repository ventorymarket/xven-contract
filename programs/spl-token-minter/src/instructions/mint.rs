use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    },
};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::ed25519_program::ID as ED25519_ID;
use anchor_lang::solana_program::sysvar::instructions::{ID as IX_ID, load_instruction_at_checked};
use crate::ed25519;
pub const ADDRESS_SIGN_BE : Pubkey = pubkey!("Cix9Y9edPfcsUoeFkm2K2srGoJQHQQKFiy8MzXtPhEsW");
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = mint_account.mint_authority.unwrap() == mint_authority.key()
    )]
    pub mint_account: Account<'info, Mint>,

    /// CHECK: PDA của program làm mint authority
    #[account(
        seeds = [b"token_mint_authority".as_ref()],
        bump,
    )]
    pub mint_authority: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

     /// CHECK:
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_token(
    ctx: Context<MintToken>, 
    message: Vec<u8>,
    signature: [u8; 64],
    amount: u64
) -> Result<()> {

    msg!("Mint: {}", &ctx.accounts.mint_account.key());
    msg!(
        "Token Address: {}",
        &ctx.accounts.associated_token_account.key()
    );


    let address_from_message = Pubkey::new_from_array((&message[0..32]).try_into().unwrap());
    let amount_from_message = u64::from_le_bytes((&message[32..40]).try_into().unwrap());
    let timestamp_from_message = i64::from_le_bytes((&message[40..48]).try_into().unwrap());
    
    msg!("address_from_message: {}",address_from_message);
    msg!("amount_from_message: {}",amount_from_message);
    msg!("timestamp_from_message: {}",timestamp_from_message);

    msg!("ix_sysvar: {}",ctx.accounts.ix_sysvar.key());

    msg!("Total instructions: {}", ctx.accounts.ix_sysvar.data_len());
    let mut found_ix_id: Option<usize> = None;
    for i in 0 .. ctx.accounts.ix_sysvar.data_len() {
        let loaded_ix = load_instruction_at_checked(i, &ctx.accounts.ix_sysvar)?;
        if loaded_ix.program_id == ED25519_ID {
            found_ix_id = Some(i);
            break;
        }
        msg!("Loaded instruction program id: {:?}", loaded_ix.program_id);
    }
    
    let ix_id = found_ix_id.ok_or_else(|| error!(CustomError::Ed25519InstructionMissing))?;

    msg!("Found ed25519 instruction at index: {}", ix_id);

    let ix: Instruction = load_instruction_at_checked(ix_id, &ctx.accounts.ix_sysvar)?;
    // Check that ix is what we expect to have been sent
    ed25519::verify_ed25519_ix(&ix, &ADDRESS_SIGN_BE.to_bytes(), &message, &signature)?;


    if address_from_message != *ctx.accounts.payer.key{
        return Err(CustomError::CallerMismatch.into());
    }

    if amount_from_message != amount {
        return Err(CustomError::AmountMismatch.into());
    }
    let current_time = Clock::get()?.unix_timestamp;
    msg!("Current time: {}", current_time);

    if !(timestamp_from_message > current_time - 10 && timestamp_from_message < current_time + 10) {
        return Err(CustomError::InvalidTimestamp.into());
    }

    let seeds = &[
        b"token_mint_authority".as_ref(),
        &[ctx.bumps.mint_authority]
    ];
    let signer = &[&seeds[..]];
    // Invoke the mint_to instruction on the token program
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer
        ),
        amount
    )?;

    emit!(MintEvent {
        token_address: ctx.accounts.mint_account.key(),
        recipient: ctx.accounts.payer.key(),
        amount,
        time_claim: timestamp_from_message as u64
    });

    msg!("Token minted successfully.");

    Ok(())
}

#[event]
pub struct MintEvent {
    pub token_address: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub time_claim: u64
}
