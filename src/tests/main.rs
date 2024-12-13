use crate::{ instructions::UpdateArgs, state::counter::CounterState, native_counter_program };
use solana_program::account_info::AccountInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{ clock::Epoch, pubkey::Pubkey };
    use std::mem;
    use borsh::{ BorshDeserialize, BorshSerialize };

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<CounterState>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );

        let accounts = vec![account];

        // Create instruction data
        let mut increment_data = vec![0]; // Increment variant
        let increment_args = UpdateArgs { value: 1 };
        increment_args.serialize(&mut increment_data).unwrap();

        let mut decrement_data = vec![1]; // Decrement variant
        let decrement_args = UpdateArgs { value: 1 };
        decrement_args.serialize(&mut decrement_data).unwrap();

        let mut update_data = vec![2]; // Update variant
        let update_args = UpdateArgs { value: 33 };
        update_args.serialize(&mut update_data).unwrap();

        let reset_data = vec![3]; // Reset variant

        // Test increment
        native_counter_program(&program_id, &accounts, &increment_data).unwrap();
        let counter = CounterState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(counter.count, 1);

        // Test decrement
        native_counter_program(&program_id, &accounts, &decrement_data).unwrap();
        let counter = CounterState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(counter.count, 0);

        // Test update
        native_counter_program(&program_id, &accounts, &update_data).unwrap();
        let counter = CounterState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(counter.count, 33);

        // Test reset
        native_counter_program(&program_id, &accounts, &reset_data).unwrap();
        let counter = CounterState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(counter.count, 0);
    }

    #[test]
    fn test_overflow_and_underflow() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<CounterState>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );

        let accounts = vec![account];

        // Test overflow
        let mut update_data = vec![2]; // Update variant
        let update_args = UpdateArgs { value: u64::MAX };
        update_args.serialize(&mut update_data).unwrap();
        native_counter_program(&program_id, &accounts, &update_data).unwrap();

        let mut increment_data = vec![0];
        let increment_args = UpdateArgs { value: 1 };
        increment_args.serialize(&mut increment_data).unwrap();
        native_counter_program(&program_id, &accounts, &increment_data).unwrap();

        let counter = CounterState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(counter.count, u64::MAX); // Should remain at max

        // Test underflow
        let reset_data = vec![3]; // Reset first
        native_counter_program(&program_id, &accounts, &reset_data).unwrap();

        let mut decrement_data = vec![1];
        let decrement_args = UpdateArgs { value: 1 };
        decrement_args.serialize(&mut decrement_data).unwrap();
        native_counter_program(&program_id, &accounts, &decrement_data).unwrap();

        let counter = CounterState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(counter.count, 0); // Should remain at 0
    }
}
