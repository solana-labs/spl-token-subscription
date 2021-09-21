use num_enum::{FromPrimitive, IntoPrimitive};

/// Enum representing the intervals a subscription may be charged at
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u64)]
pub enum Interval {
    /// No interval defined (the subscription can be charged at any time)
    #[num_enum(default)]
    None = 0,
    /// Milliseconds in an hour
    Hourly = 3_600_000,
    /// Milliseconds in a day
    Daily = 86_400_000,
    /// Milliseconds in a week
    Weekly = 604_800_000,
    /// Milliseconds in a month
    Monthly = 2_629_800_000,
    /// Milliseconds in a year
    Yearly = 31_557_600_000,
}

impl Default for Interval {
    fn default() -> Self {
        Interval::None
    }
}
