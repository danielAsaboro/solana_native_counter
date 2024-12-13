use crate::state::counter::CounterState;

pub fn initialize_counter() -> Result<CounterState, ()> {
    Ok(CounterState::new())
}
