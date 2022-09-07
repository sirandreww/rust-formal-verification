// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Clause;
use std::collections::HashSet;
use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct CNF {
    clauses : HashSet<Clause>
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl CNF {
    pub fn add_clause(&mut self, new_clause : &Clause){
        self.clauses.insert((*new_clause).to_owned());
    }

    pub fn get_number_of_clauses(&self) -> usize {
        return self.clauses.len();
    }
}

impl Default for CNF {
    fn default() -> Self {
        Self { clauses: (HashSet::new()) }
    }
}

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_vec = self.clauses
        .iter()
        .map(|lit| lit.to_string())
        .collect::<Vec<String>>();
        return write!(f, "({})", string_vec.join(" & "));
    }
}