use {super::*, solana_program::pubkey::Pubkey};

/// Merchant account state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Merchant {
    /// Account type, must be MerchantV1 currently
    pub account_type: AccountType,
    /// Authority that can manage products and prices
    pub authority: Pubkey,
    /// URI for metadata about the merchant
    pub uri: String,
}
