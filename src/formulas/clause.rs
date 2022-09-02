// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Clause {
    literals : Vec<Literal>
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Clause {
    pub fn new(literals : Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn add_literal(&mut self, new_literal : &Literal) {
        self.literals.push((*new_literal).to_owned());
    }
}

impl Default for Clause {
    fn default() -> Self {
        Self { literals: Vec::new() }
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_vec = self.literals
        .iter()
        .map(|lit| lit.to_string())
        .collect::<Vec<String>>();
        return write!(f, "({})", string_vec.join(" | "));
    }
}