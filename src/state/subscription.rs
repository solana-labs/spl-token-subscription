use {
    super::*,
    num_enum::{FromPrimitive, IntoPrimitive},
    solana_program::{clock::UnixTimestamp, pubkey::Pubkey},
};

/// Subscription account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Subscription {
    /// Account type, must be SubscriptionV1 currently
    pub account_type: AccountType,
    /// Bump seed for subscription PDA of [b"subscription", product_key, authority_key]
    pub seed: u8,
    /// Status of the subscription
    pub status: SubscriptionStatus,
    /// Number of tokens that can be charged for the subscription each interval
    pub amount: u64,
    /// Cumulative amount of tokens that have been paid for the subscription
    pub paid: u64,
    /// URI for metadata about the subscription
    pub uri: String,
    /// Time the subscription was created
    pub created: UnixTimestamp,
    /// Number of charges that have been created (each charge pubkey is a PDA with this index as a seed)
    pub charges: u64,
}

/// Enum representing the statuses a subscription can have
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum SubscriptionStatus {
    /// The subscription is inactive
    #[num_enum(default)]
    Inactive,
    /// The subscription is active
    Active,
    /// The subscription is canceled
    Canceled,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        SubscriptionStatus::Inactive
    }
}
