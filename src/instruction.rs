//! Instruction types

use crate::state::*;

/// Instructions supported by the program
#[derive(Clone, Debug, PartialEq)]
pub enum StreamInstruction {
    /// Start a stream
    StartStream {
        /// Amount of tokens that can be withdrawn each interval
        amount: u64,
        /// Cumulative amount of tokens that have been withdrawn
        interval: Interval,
    },
    /// Stop a stream
    StopStream,
    /// Withdraw from a stream
    Withdraw {
        /// Amount of tokens to withdraw, or u64::MAX to withdraw max
        amount: u64,
    },
}
