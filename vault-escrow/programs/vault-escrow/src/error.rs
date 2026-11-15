use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("The vault does not have enough lamports for this withdrawal.")]
    InsufficientVaultFunds,
}
