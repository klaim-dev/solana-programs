use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Deposit amount must be greater than zero")]
    InvalidDepositAmount,
}
