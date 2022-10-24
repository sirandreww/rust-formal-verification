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
    max_variable_number: u32,
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
        // this should work since all variable numbers must have highest bit be 0.
        self.max_variable_number.try_into().unwrap()
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

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
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

// impl IntoIterator for Clause {
//     type Item = Literal;
//     type IntoIter = <Vec<Literal> as IntoIterator>::IntoIter; // so that you don't have to write std::vec::IntoIter, which nobody remembers anyway

//     fn into_iter(self) -> Self::IntoIter {
//         self.literals.into_iter()
//     }
// }

//   // We deref to slice so that we can reuse the slice impls
//   impl Deref for BinaryVec {
//     type Output = [u8];

//     fn deref(&self) -> &[u8] {
//       &self.vec[..]
//     }
//   }
