// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use std::cmp::max;
use std::fmt;
use std::hash::Hash;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Clause {
    literals: Vec<Literal>,
    max_variable_number: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Clause {
    pub fn new(mut literals: Vec<Literal>) -> Self {
        literals.sort();
        let max_variable_number = literals.iter().map(|lit| lit.get_number()).max();
        match max_variable_number {
            // input not empty
            Some(x) => Self {
                literals: literals.to_vec(),
                max_variable_number: x,
            },
            // input empty
            None => Self {
                literals: literals.to_vec(),
                max_variable_number: 0,
            },
        }
    }

    pub fn add_literal(&mut self, new_literal: &Literal) {
        self.literals.push((*new_literal).to_owned());
        self.literals.sort();
        self.max_variable_number = max(new_literal.get_number(), self.max_variable_number)
    }

    pub fn get_highest_variable_number(&self) -> i32 {
        self.max_variable_number
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
