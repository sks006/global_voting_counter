use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// Declare the internal components
pub mod processor;
pub mod state;
pub mod error;
pub mod instruction;

use crate::processor::Processor;

// The Ignition Switch: This macro handles the low-level boiler plate
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Pass the signal directly to the Engine's main controller
    Processor::process(program_id, accounts, instruction_data)
}