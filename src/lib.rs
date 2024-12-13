use borsh::{ BorshDeserialize, BorshSerialize };
use instructions::decrement::decrement_counter;
use instructions::increment::increment_counter;
use instructions::reset::reset_counter;
use instructions::update::update_counter;
use instructions::CounterInstructions;
use solana_program::entrypoint;
use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::{ AccountInfo, next_account_info },
    msg,
};
use solana_program::program_error::ProgramError;
use state::counter::CounterState;

pub mod instructions;
pub mod state;
pub mod constants;

entrypoint!(native_counter_program);

pub fn native_counter_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Counter program entry point");
    // Parse instruction
    let instruction: CounterInstructions =
        CounterInstructions::unpack_instruction(instruction_data)?;

    // Get account iterator
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Validate account ownership
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Verify account is writable
    if !account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize account data
    let mut counter_account = CounterState::try_from_slice(&account.data.borrow())?;

    // Process instruction
    let result = match instruction {
        CounterInstructions::Increment(args) => {
            increment_counter(args.value, &mut counter_account)
        }
        CounterInstructions::Decrement(args) => {
            decrement_counter(args.value, &mut counter_account)
        }
        CounterInstructions::Reset => { reset_counter(&mut counter_account) }
        CounterInstructions::Update(args) => { update_counter(args.value, &mut counter_account) }
    };

    match result {
        Ok(_) => {
            counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
            Ok(())
        }
        Err(e) => {
            msg!("Error processing instruction: {:?}", e);
            Err(ProgramError::Custom(1))
        }
    }
}
