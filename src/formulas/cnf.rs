// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Clause;
use crate::formulas::Variable;
use std::cmp::max;
use std::collections::HashSet;
use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default)]
pub struct CNF {
    clauses: HashSet<Clause>,
    max_variable_number: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl CNF {
    pub fn add_clause(&mut self, new_clause: &Clause) {
        self.max_variable_number = max(
            self.max_variable_number,
            new_clause.get_highest_variable_number(),
        );
        self.clauses.insert((*new_clause).to_owned());
    }

    pub fn get_number_of_clauses(&self) -> usize {
        self.clauses.len()
    }

    pub fn get_new_variable(&mut self) -> Variable {
        Variable::new(self.max_variable_number + 1)
    }

    pub fn get_highest_variable_number(&self) -> i32 {
        self.max_variable_number
    }
}

// ************************************************************************************************
// default constructor
// ************************************************************************************************

// impl Default for CNF {
//     fn default() -> Self {
//         Self {
//             clauses: Default::default(),
//             max_variable_number: 0,
//         }
//     }
// }

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_vec = self
            .clauses
            .iter()
            .map(|lit| lit.to_string())
            .collect::<Vec<String>>();
        string_vec.sort();
        write!(f, "({})", string_vec.join(" & "))
    }
}
