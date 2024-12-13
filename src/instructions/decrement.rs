use crate::state::counter::CounterState;

pub fn decrement_counter(user_input: u64, counter: &mut CounterState) -> Result<&CounterState, ()> {
    // Check for overflow
    if let Some(new_count) = counter.count.checked_sub(user_input) {
        counter.count = new_count;
        Ok(counter)
    } else {
        counter.count = 0;
        Ok(counter)
    }
}
