use num_enum::{FromPrimitive, IntoPrimitive};

/// Enum representing the account versions managed by the program
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Version {
    /// If the account has not been initialized, the value will be 0
    #[num_enum(default)]
    Uninitialized,
    /// Stream
    StreamV1,
}

impl Default for Version {
    fn default() -> Self {
        Version::Uninitialized
    }
}
