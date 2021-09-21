#![deny(missing_docs)]

//! A token subscription program for the Solana blockchain

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("3URVKV5ptvLWpXUritF7qCkwcKx3Uep4LrwXi6LnZfmc");
