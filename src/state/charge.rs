use {
    super::*,
    solana_program::{clock::UnixTimestamp, pubkey::Pubkey},
};

/// A charge represents a successful payment order issued by a merchant against a subscription

/// Charge account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Charge {
    /// Account type, must be ChargeV1 currently
    pub account_type: AccountType,
    /// Bump seed for Charge PDA of [b"charge", subscription_key, index]
    pub seed: u8,
    /// Number of tokens that were charged
    pub amount: u64,
    /// Time the charge was created
    pub created: UnixTimestamp,
}
