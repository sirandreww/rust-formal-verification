// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use crate::formulas::CNF;
use std::fmt;
use std::hash::Hash;
use std::ops::Not;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct Clause {
    literals: Vec<Literal>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Clause {
    pub fn new(literals: &[Literal]) -> Self {
        let mut sorted = literals.to_owned();
        sorted.sort();
        Self {
            literals: sorted,
        }
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

impl Not for Clause {
    type Output = CNF;

    fn not(self) -> Self::Output {
        let mut cnf = CNF::new();
        for lit in self.iter() {
            cnf.add_clause(&Clause::new(&[!lit.to_owned()]))
        }
        cnf
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut literals = self.literals.to_owned();
        literals.sort();
        let string_vec = literals
            .iter()
            .map(|lit| lit.to_string())
            .collect::<Vec<String>>();
        write!(f, "({})", string_vec.join(" | "))
    }
}
