use num_enum::{FromPrimitive, IntoPrimitive};

/// Enum representing the intervals a subscription may be charged at
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u64)]
pub enum Interval {
    /// No interval defined (the subscription can be charged at any time)
    #[num_enum(default)]
    None = 0,
    /// Milliseconds in a second
    Second = 1_000,
    /// Milliseconds in a minute
    Minute = 60_000,
    /// Milliseconds in an hour
    Hour = 3_600_000,
    /// Milliseconds in a day
    Day = 86_400_000,
    /// Milliseconds in a week
    Week = 604_800_000,
    /// Milliseconds in a month
    Month = 2_629_800_000,
    /// Milliseconds in a year
    Year = 31_557_600_000,
}

impl Default for Interval {
    fn default() -> Self {
        Interval::None
    }
}
