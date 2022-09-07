// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use std::{fmt, collections::HashSet};
use std::hash::{Hash, Hasher};
use std::collections::BinaryHeap;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, PartialEq, Clone)]
pub struct Clause {
    literals : HashSet<Literal>
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Clause {
    pub fn new(literals : HashSet<Literal>) -> Self {
        Self { literals }
    }

    pub fn add_literal(&mut self, new_literal : &Literal) {
        self.literals.insert((*new_literal).to_owned());
    }
}

impl Default for Clause {
    fn default() -> Self {
        Self { literals: HashSet::new() }
    }
}

impl Hash for Clause {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut vec_sorted = self.literals.iter().collect::<Vec<_>>();
        vec_sorted.sort();
        for lit in vec_sorted {
            lit.hash(state);
        }
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec_sorted = self.literals.iter().collect::<Vec<_>>();
        vec_sorted.sort();
        let string_vec = vec_sorted
        .iter()
        .map(|lit| lit.to_string())
        .collect::<Vec<String>>();
        return write!(f, "({})", string_vec.join(" | "));
    }
}