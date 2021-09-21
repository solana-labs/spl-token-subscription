use {
    num_enum::{FromPrimitive, IntoPrimitive},
    solana_program::pubkey::Pubkey,
    super::*,
};

/// Product account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Product {
    /// Account type, must be ProductV1 currently
    pub account_type: AccountType,
    /// Merchant account the product belongs to
    pub merchant: Pubkey,
    /// Status of the Product
    pub status: ProductStatus,
    /// Interval a subscription for the product can be charged on
    pub interval: Interval,
    /// Token mint the price is denominated in
    pub mint: Pubkey,
    /// Amount of tokens that can be charged for a subscription
    pub amount: u64,
    /// URI for metadata about the product
    pub uri: String,
}

/// Enum representing the statuses a product can have
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum ProductStatus {
    /// The product is not active
    #[num_enum(default)]
    Inactive,
    /// The product is active
    Active,
}

impl Default for ProductStatus {
    fn default() -> Self {
        ProductStatus::Inactive
    }
}
