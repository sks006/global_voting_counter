use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum VotingError {
    #[error("Unauthorized: Signer is not the Admin")]
    Unauthorized,

    #[error("Counter is paused: No votes allowed")]
    CounterPaused,

    #[error("Already voted: This user was the last voter")]
    AlreadyVoted,

    #[error("Invalid Account Tag: Type mismatch detected")]
    InvalidAccountTag,

    #[error("Math Overflow: The counter has reached its limit")]
    Overflow,
}

impl From<VotingError> for ProgramError {
    fn from(e: VotingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}