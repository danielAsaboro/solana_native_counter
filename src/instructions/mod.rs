use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::program_error::ProgramError;

pub mod decrement;
pub mod increment;
pub mod initialize;
pub mod reset;
pub mod update;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u64,
}
pub enum CounterInstructions {
    Increment(UpdateArgs),
    Decrement(UpdateArgs),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack_instruction(input: &[u8]) -> Result<Self, ProgramError> {
        //
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(UpdateArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(UpdateArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => {
                return Err(ProgramError::InvalidInstructionData);
            }
        })
    }
}
