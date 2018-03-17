pub mod model;
use turing::model::*;
#[derive(Clone)]
pub struct Machine{
    pub tape: Tape,
    pub states: Vec<State>,
    pub current_state: Option<usize>,
}

impl Machine {
    pub fn build_new(tape: Tape, states: Vec<State>) -> Machine {
        Machine {
            tape,
            states,
            current_state: Some(0),
        }
    }

    pub fn step(& mut self) {
       // let s = &self.states[self.current_state];
       match self.current_state {
           Some(s) => {
               self.current_state = 
                   match self.states[s].rules.get(&self.tape.get_char())
                   {
                       Some(x) => Some(self.tape.move_head(x)),
                       None => {
                       println!("There is no state for symbol {}", self.tape.get_char());
                       None
                       },
                   };
           },
           None => println!("There is no rules")
       }
    }

    //format q0:c->cr1
    pub fn parse_states(state_str: & str) -> Vec<State> {
            if c == 'q' {
                
            }
    }
}
