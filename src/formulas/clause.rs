// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use std::fmt;
use std::hash::Hash;
use std::ops::Not;

use super::Cube;
use super::literal::VariableType;

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
        Self { literals: sorted }
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

    pub fn get_highest_variable_number(&self) -> VariableType{
        match self.literals.last(){
            Some(l) => l.get_number(),
            None => 0,
        }
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

impl Not for Clause {
    type Output = Cube;

    fn not(self) -> Self::Output {
        let mut literals = Vec::new();
        for lit in self.iter() {
            literals.push(!lit.to_owned());
        }
        Cube::new(&literals)
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
        write!(f, "{} 0", string_vec.join(" "))
    }
}
