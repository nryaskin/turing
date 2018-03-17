use std::collections::HashMap;

#[derive(Clone)]
pub struct Tape {
   pub tape: Vec<char>,
   pub  head: usize,
}

impl Tape {
    pub fn get_char(&self) -> char {
        self.tape[self.head]
    }

    pub fn move_head(& mut self, rule: &Rule) -> usize {
        //TODO:Write macro for this shit
        match rule {
           &Rule::Left(rep, i) => { 
                self.tape[self.head] = rep;
                if self.head < 1 {
                    self.tape.insert(0,'#');
                    self.head = 0;
                } else {
                    self.head = self.head - 1;
                }
                i},
           &Rule::Right(rep, i) =>  { 
                self.tape[self.head] = rep;
                self.head = self.head + 1;
                if self.head == self.tape.len(){
                    self.tape.push('#');
                }
                i}, 
        }
    }

}
#[derive(Clone)]
pub struct State {
    pub rules: HashMap<char, Rule>,
}

#[derive(Clone)]
pub enum Rule {
    Left(char, usize),
    Right(char, usize),
}
