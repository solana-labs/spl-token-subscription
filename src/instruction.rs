//! Instruction types

/// Instructions supported by the program.
#[derive(Clone, Debug, PartialEq)]
pub enum SubscriptionInstruction {
    /// Create a merchant
    CreateMerchant,
    /// Update a merchant (set authority)
    UpdateMerchant,

    /// Create a product
    CreateProduct,
    /// Update a product (amount, uri)
    UpdateProduct,
    /// Activate a product (set status to active)
    ActivateProduct,
    /// Deactivate a product (set status to inactive)
    DeactivateProduct,

    /// Create a subscription
    CreateSubscription,
    /// Update a subscription (amount, uri)
    UpdateSubscription,
    /// Cancel a subscription
    CancelSubscription,

    /// Create a charge
    CreateCharge,
}
