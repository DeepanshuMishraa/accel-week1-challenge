use anchor_lang::prelude::*;

#[error_code]
pub enum TransferHookError {
    #[msg("Transfer hook was invoked outside of a transfer")]
    NotTransferring,
    #[msg("Transfer must involve the vault token account")]
    TransferMustTouchVault,
    #[msg("Source owner is not whitelisted for vault deposit")]
    SourceNotWhitelisted,
    #[msg("Destination owner is not whitelisted for vault withdrawal")]
    DestinationNotWhitelisted,
    #[msg("Mint does not match the vault mint")]
    InvalidVaultMint,
}
