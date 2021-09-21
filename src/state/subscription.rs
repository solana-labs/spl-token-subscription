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
    /// Status of the Subscription
    pub status: SubscriptionStatus,
    /// Pubkey of the Product account
    pub product: Pubkey,
    /// Pubkey of the Charge account
    pub charge: Pubkey,
    /// Token mint the price is denominated in, set at the time the subscription is created
    pub mint: Pubkey,
    /// Amount of tokens the price is denominated in, set at the time the subscription is created
    pub amount: u64,
    /// Token account that will be charged
    pub account: Pubkey,
    /// Authority that can manage the subscription
    pub authority: Pubkey,
    /// URI for metadata about the subscription
    pub uri: String,
    /// Time the subscription was created
    pub timestamp: UnixTimestamp,
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
