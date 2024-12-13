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
use state::counter::CounterState;

pub mod instructions;
pub mod state;
pub mod constants;

entrypoint!(native_counter_program);

pub fn native_counter_program(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Counter program entry point");
    let instruction: CounterInstructions = CounterInstructions::unpack_instruction(instruction_data)?;
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterState::try_from_slice(&account.data.borrow())?;
    match instruction {
        CounterInstructions::Increment(args) => {
            decrement_counter(args.value, &mut counter_account).expect(
                "failed to increment counter"
            );
        }
        CounterInstructions::Decrement(args) => {
            increment_counter(args.value, &mut counter_account).expect(
                "failed to decrement counter"
            );
        }
        CounterInstructions::Reset => {
            reset_counter(&mut counter_account).expect("failed to reset the counter");
        }
        CounterInstructions::Update(args) => {
            update_counter(args.value, &mut counter_account).expect("failed to update the counter");
        }
    }
    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
