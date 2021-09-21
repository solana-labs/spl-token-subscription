//! Instruction types

/// Instructions supported by the program.
#[derive(Clone, Debug, PartialEq)]
pub enum SubscriptionInstruction {
    /// Create a merchant
    CreateMerchant,

    /// Create a product
    CreateProduct,
    /// Update a product
    UpdateProduct,

    /// Create a subscription
    CreateSubscription,
    /// Cancel a subscription
    CancelSubscription,

    /// Create a charge
    CreateCharge,
}
