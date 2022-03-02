use std::convert::Into;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;
pub enum EscrowInstruction {
    // Starts the trade by creating and populating an escrow account and
    // transfering ownership of the token to the given temp token account
    // to the PDA

    // Accounts expects:
    //
    // 0. `[signer]` the account of the person initializing the escrow
    
    // 1. `[writable]` Temporary token account that should be created
    // prior to this instruction and owned by the initializer
    
    // 2. `[]` the initializer token account for the token they will receive
    // should the trade go through
    
    // 3. `[writable]` The escrow account, it will hold the necessary
    // info about the trade
    
    // 4. `[]` the rent sysvar
    
    // 5. `[]` the token program
    InitEscrow {
        // the amount party A expects to receive from party B
        amount: u64
    }
}

impl EscrowInstruction {
    // unpacks a byte buffer into a [EscrowInstruction]
    //(enum.EscrowInstruction.html)
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into())
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}