// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use std::fmt;
use std::hash::Hash;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, PartialEq, Clone, Hash, Default)]
pub struct Clause {
    literals: Vec<Literal>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Clause {
    pub fn new(literals: &mut [Literal]) -> Self {
        literals.sort();
        Self {
            literals: literals.to_vec(),
        }
    }

    pub fn add_literal(&mut self, new_literal: &Literal) {
        self.literals.push((*new_literal).to_owned());
        self.literals.sort();
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_vec = self
            .literals
            .iter()
            .map(|lit| lit.to_string())
            .collect::<Vec<String>>();
        write!(f, "({})", string_vec.join(" | "))
    }
}
