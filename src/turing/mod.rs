pub mod model;
use turing::model::*;
#[derive(Clone)]
pub struct Machine{
    pub tape: Tape,
    pub states: Vec<State>,
    pub current_state: usize,
}

impl Machine {
    pub fn build_new(tape: Tape, states: Vec<State>) -> Machine {
        Machine {
            tape,
            states,
            current_state: 0,
        }
    }

    pub fn step(& mut self) {
       // let s = &self.states[self.current_state];
       self.current_state = self.tape.move_head(&self.states[self.current_state].rule);
    }
}
