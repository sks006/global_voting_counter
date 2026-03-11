use solana_program::program_error::ProgramError;
use std::convert::TryInto;
use crate::error::VotingError;

pub enum CounterInstruction {
    /// 0: Initializes the AdminConfig (The ECU)
    /// Accounts: [signer, pda_admin_config, system_program]
    InitializeAdmin,

    /// 1: Initializes a specific Counter (The Odometer)
    /// Accounts: [signer, pda_counter, pda_admin_config, system_program]
    InitializeCounter {
        count: u64,
    },

    /// 2: Increments the count (The Gas Pedal)
    /// Accounts: [signer, pda_counter, pda_admin_config]
    Increment
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Split the first byte (Tag/Gear) from the rest of the data
        let (&tag, rest) = input.split_first().ok_or(VotingError::InvalidAccountTag)?;

        Ok(match tag {
            0 => Self::InitializeAdmin,
            1 => {
                // For InitializeCounter, we need to extract 8 bytes for the 'u64' count
                let count = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(VotingError::InvalidAccountTag)?; // Reuse error for malformed data
                Self::InitializeCounter { count }
            },
            2 => Self::Increment,
            _ => return Err(VotingError::InvalidAccountTag.into()),
        })
    }
}