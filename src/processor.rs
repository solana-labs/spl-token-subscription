//! Program state processor

use {
    crate::instruction::StreamInstruction,
    crate::state::Interval,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = StreamInstruction::unpack(input)?;
    match instruction {
        StreamInstruction::StartStream { amount, interval } => {
            msg!("Instruction: StartStream");
            process_start_stream(program_id, amount, interval, accounts)
        }
        StreamInstruction::StopStream => {
            msg!("Instruction: StopStream");
            process_stop_stream(program_id, accounts)
        }
    }
}

#[inline(never)] // avoid stack frame limit
fn process_start_stream(
    program_id: &Pubkey,
    amount: u64,
    interval: Interval,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    Ok(())
}

#[inline(never)] // avoid stack frame limit
fn process_stop_stream(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    Ok(())
}
