use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::ed25519_program::ID as ED25519_ID;
use crate::errors::CustomError;

pub fn verify_ed25519_ix(
    ix: &Instruction,
    pubkey: &[u8],
    msg: &[u8],
    sig: &[u8],
) -> Result<()> {

    msg!("program_id {}", ix.program_id);
    msg!("ED25519_ID {}", ED25519_ID);
    msg!("ix.accounts.len() {}", ix.accounts.len());
    msg!("ix.data.len() {}", ix.data.len());
    msg!("data {}",16 + 64 + 32 + msg.len());

    if  ix.program_id != ED25519_ID     ||
        ix.accounts.len() != 0          ||
        ix.data.len() != (16 + 64 + 32 + msg.len())
    {
        msg!("mess 1");
        return Err(CustomError::SignatureVerificationFailed.into());
    }

    check_ed25519_data(&ix.data, pubkey, msg, sig)?;

    Ok(())
}

/// Verify serialized Ed25519Program instruction data
pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33

    // "Deserializing" byte slices

    let num_signatures               = &[data[0]];        // Byte  0
    let padding                      = &[data[1]];        // Byte  1
    let signature_offset                = &data[2..=3];      // Bytes 2,3
    let signature_instruction_index     = &data[4..=5];      // Bytes 4,5
    let public_key_offset               = &data[6..=7];      // Bytes 6,7
    let public_key_instruction_index    = &data[8..=9];      // Bytes 8,9
    let message_data_offset             = &data[10..=11];    // Bytes 10,11
    let message_data_size               = &data[12..=13];    // Bytes 12,13
    let message_instruction_index       = &data[14..=15];    // Bytes 14,15

    let data_pubkey                     = &data[16..16+32];  // Bytes 16..16+32
    let data_sig                        = &data[48..48+64];  // Bytes 48..48+64
    let data_msg                        = &data[112..];      // Bytes 112..end

    // Expected values

    let exp_public_key_offset:      u16 = 16; // 2 * u8 + 7 * u16
    let exp_signature_offset:       u16 = exp_public_key_offset + pubkey.len() as u16;
    let exp_message_data_offset:    u16 = exp_signature_offset + sig.len() as u16;
    let exp_num_signatures:          u8 = 1;
    let exp_message_data_size:      u16 = msg.len().try_into().unwrap();

    // Header and Arg Checks

    // Header
    if  num_signatures                  != &exp_num_signatures.to_le_bytes()        ||
        padding                         != &[0]                                     ||
        signature_offset                != &exp_signature_offset.to_le_bytes()      ||
        signature_instruction_index     != &u16::MAX.to_le_bytes()                  ||
        public_key_offset               != &exp_public_key_offset.to_le_bytes()     ||
        public_key_instruction_index    != &u16::MAX.to_le_bytes()                  ||
        message_data_offset             != &exp_message_data_offset.to_le_bytes()   ||
        message_data_size               != &exp_message_data_size.to_le_bytes()     ||
        message_instruction_index       != &u16::MAX.to_le_bytes()  
    {
        msg!("mess 2");
        return Err(CustomError::SignatureVerificationFailed.into());
    }

    // Arguments
    if  data_pubkey != pubkey   ||
        data_msg    != msg      ||
        data_sig    != sig
    {
        msg!("mess 3");
        return Err(CustomError::SignatureVerificationFailed.into());
    }

    Ok(())
}