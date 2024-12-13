use crate::state::counter::CounterState;

pub fn update_counter(user_input: u64, counter: &mut CounterState) -> Result<&CounterState, ()> {
    counter.count = user_input;
    Ok(counter)
}
