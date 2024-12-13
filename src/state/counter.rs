use borsh::{ BorshDeserialize, BorshSerialize };

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterState {
    pub count: u64,
}

impl CounterState {
    pub fn new() -> Self {
        CounterState { count: 0 }
    }
}
