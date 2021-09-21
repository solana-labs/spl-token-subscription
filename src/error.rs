//! Error types

use {
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the program
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SubscriptionError {
    // 0
    /// InstructionUnpackError
    #[error("InstructionUnpackError")]
    InstructionUnpackError,
    /// UnspecifiedError
    #[error("UnspecifiedError")]
    UnspecifiedError,
    /// MathError
    #[error("MathError")]
    MathError,
}

impl From<SubscriptionError> for ProgramError {
    fn from(e: SubscriptionError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SubscriptionError {
    fn type_of() -> &'static str {
        "Subscribe Error"
    }
}

impl PrintProgramError for SubscriptionError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
