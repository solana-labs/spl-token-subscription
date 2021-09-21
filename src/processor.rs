//! Program state processor

use {
    crate::instruction::SubscriptionInstruction,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = SubscriptionInstruction::unpack(input)?;
    match instruction {
        SubscriptionInstruction::CreateMerchant => {
            msg!("Instruction: CreateMerchant");
            process_create_merchant(program_id, accounts)
        }
    }
}

#[inline(never)] // avoid stack frame limit
fn process_create_merchant(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    Ok(())
}
