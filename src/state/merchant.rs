use {super::*, solana_program::pubkey::Pubkey};

/// A merchant represents a seller of products that can be subscribed to

/// Merchant account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Merchant {
    /// Account type, must be MerchantV1 currently
    pub account_type: AccountType,
    /// Authority that can manage products
    pub authority: Pubkey,
    /// URI for metadata about the merchant
    pub uri: String,
    /// Number of products that have been created (each product pubkey is a PDA with this index as a seed)
    pub products: u64,
}
