use {
    super::*,
    num_enum::{FromPrimitive, IntoPrimitive},
    solana_program::{clock::UnixTimestamp, pubkey::Pubkey},
};

/// A stream represents a stream of payments from an account to another account

/// Stream account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stream {
    /// Account version, must be StreamV1 currently
    pub version: Version,
    /// Bump seed for stream PDA of [b"stream", token_account, withdraw_authority]
    pub seed: u8,
    /// Amount of tokens that can be withdrawn each interval
    pub amount: u64,
    /// Cumulative amount of tokens that have been withdrawn
    pub total: u64,
    /// A stream allows `amount` tokens to be withdrawn every `interval`
    pub interval: Interval,
    /// Time the stream was started
    pub start: UnixTimestamp,
}
