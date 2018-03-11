use std::collections::HashMap;

#[derive(Clone)]
pub struct Tape {
    pub tape: Vec<char>,
    pub head: usize,
}

impl Tape {
    pub fn move_head(& mut self, rule: &Rule) -> usize {
        //TODO:Write macro for this shit
        match rule {
           &Rule::Left(rep, i) => { 
                self.tape[self.head] = rep;
                self.head = self.head - 1;
                i},
           &Rule::Right(rep, i) =>  { 
                self.tape[self.head] = rep;
                self.head = self.head + 1;
                i}, 
        }
    }

}
#[derive(Clone)]
pub struct State {
    pub rule: Rule, //HashMap<String, Rule>,
}

#[derive(Clone)]
pub enum Rule {
    Left(char, usize),
    Right(char, usize),
}
