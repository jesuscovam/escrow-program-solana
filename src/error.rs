use thiserror::Error;
use solana_program::program_error::ProgramError;
#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    // rent not covered
    #[error("Not rent excempt")]
    NotRentExempt
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}