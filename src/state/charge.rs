use {
    super::*,
    solana_program::{clock::UnixTimestamp, pubkey::Pubkey},
};

/// Charge account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Charge {
    /// Account type, must be ChargeV1 currently
    pub account_type: AccountType,
    /// Subscription that was charged
    pub subscription: Pubkey,
    /// Mint of the token the charge is denominated in, set at the time the charge is created
    pub mint: Pubkey,
    /// Amount of the charge
    pub amount: u64,
    /// Time the charge was created
    pub timestamp: UnixTimestamp,
}
