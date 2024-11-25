use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Failed to not owner call")]
    NotOnwerCall,

    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,

    #[msg("Caller does not match")]
    CallerMismatch,

    #[msg("Amount does not match")]
    AmountMismatch,

    #[msg("Invalid timestamp")]
    InvalidTimestamp,

    #[msg("Invalid timestamp")]
    Ed25519InstructionMissing
}
