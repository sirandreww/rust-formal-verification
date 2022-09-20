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
    pub fn new(literals: &[Literal]) -> Self {
        if literals.is_empty() {
            Self {
                literals: vec![],
                max_variable_number: 0,
            }
        } else {
            let mut sorted_lits = literals.to_owned();
            sorted_lits.sort();
            let biggest_lit = sorted_lits[sorted_lits.len() - 1];
            Self {
                literals: sorted_lits,
                max_variable_number: biggest_lit.get_number(),
            }
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

    pub fn to_vector_of_numbers(&self) -> Vec<i32> {
        self.literals
            .iter()
            .map(|one_literal| one_literal.to_dimacs_number())
            .collect::<Vec<i32>>()
    }

    pub fn to_dimacs_line(&self) -> String {
        let mut string_vec = self
            .literals
            .iter()
            .map(|one_literal| one_literal.to_dimacs_number().to_string())
            .collect::<Vec<String>>();
        string_vec.push("0".to_string());
        string_vec.join(" ")
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
