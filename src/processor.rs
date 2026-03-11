use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::instruction::CounterInstruction;
use crate::error::VotingError;
use crate::state::{Counter, AdminConfig, TAG_COUNTER, TAG_ADMIN_CONFIG, TAG_UNINITIALIZED};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // 1. GEARBOX: Unpack raw bytes into structured Enum
        let instruction = CounterInstruction::unpack(instruction_data)?;

        // 2. DISPATCH: Shift into the correct handler logic
        match instruction {
            CounterInstruction::InitializeAdmin => {
                msg!("Instruction: Initialize Admin");
                Self::process_initialize_admin(accounts, program_id)
            }
            CounterInstruction::InitializeCounter { count } => {
                msg!("Instruction: Initialize Counter");
                Self::process_initialize_counter(accounts, count, program_id)
            }
            CounterInstruction::Increment => {
                msg!("Instruction: Increment");
                Self::process_increment(accounts, program_id)
            }
        }
    }

    fn process_initialize_admin(
        accounts: &[AccountInfo], 
        _program_id: &Pubkey
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let admin_signer = next_account_info(account_info_iter)?;
        let admin_config_account = next_account_info(account_info_iter)?;

        if !admin_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut admin_data = admin_config_account.data.borrow_mut();
        if admin_data[0] != TAG_UNINITIALIZED {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let config = AdminConfig {
            tag: TAG_ADMIN_CONFIG,
            admin: *admin_signer.key,
            is_paused: false,
        };

        config.serialize(&mut &mut admin_data[..])?;
        msg!("AdminConfig (ECU) installed.");
        Ok(())
    }

    fn process_initialize_counter(
        accounts: &[AccountInfo], 
        count: u64, 
        _program_id: &Pubkey
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;
        let counter_account = next_account_info(account_info_iter)?;

        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut counter_data = counter_account.data.borrow_mut();
        if counter_data[0] != TAG_UNINITIALIZED {
             return Err(ProgramError::AccountAlreadyInitialized);
        }

        let new_counter = Counter {
            tag: TAG_COUNTER,
            count,
            last_voter: Pubkey::default(),
        };

        new_counter.serialize(&mut &mut counter_data[..])?;
        msg!("Odometer (Counter) manufactured at value: {}", count);
        Ok(())
    }

    fn process_increment(
        accounts: &[AccountInfo], 
        _program_id: &Pubkey
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let voter = next_account_info(account_info_iter)?;
        let counter_account = next_account_info(account_info_iter)?;
        let admin_config_account = next_account_info(account_info_iter)?;

        if !voter.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Inspection: Check Admin (ECU) status
        let admin_config = AdminConfig::try_from_slice(&admin_config_account.data.borrow())?;
        if admin_config.tag != TAG_ADMIN_CONFIG {
            return Err(VotingError::InvalidAccountTag.into());
        }
        if admin_config.is_paused {
            return Err(VotingError::CounterPaused.into());
        }

        // Inspection: Check Counter (Odometer) status
        let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
        if counter_data.tag != TAG_COUNTER {
            return Err(VotingError::InvalidAccountTag.into());
        }

        // Anti-Spam: One vote per person per block
        if counter_data.last_voter == *voter.key {
            return Err(VotingError::AlreadyVoted.into());
        }

        // Combustion: Mutation
        counter_data.count = counter_data.count
            .checked_add(1)
            .ok_or(VotingError::Overflow)?;
        
        counter_data.last_voter = *voter.key;

        // Persistence: Write back to account
        counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

        msg!("Odometer advanced. New reading: {}", counter_data.count);
        Ok(())
    }
}