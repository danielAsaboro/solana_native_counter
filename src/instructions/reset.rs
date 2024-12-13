use crate::state::counter::CounterState;

pub fn reset_counter(counter: &mut CounterState) -> Result<&CounterState, ()> {
    let updated_counter = counter;
    updated_counter.count = 0;
    Ok(updated_counter)
}
