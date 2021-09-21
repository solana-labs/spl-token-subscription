use num_enum::{FromPrimitive, IntoPrimitive};

/// Enum representing the account types managed by the program
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum AccountType {
    /// If the account has not been initialized, the value will be 0
    #[num_enum(default)]
    Uninitialized,
    /// Merchant
    MerchantV1,
    /// Charge
    ChargeV1,
    /// Product
    ProductV1,
    /// Subscription
    SubscriptionV1,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Uninitialized
    }
}
