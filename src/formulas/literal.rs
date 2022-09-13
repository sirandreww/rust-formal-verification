// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Variable;
use std::{fmt, ops::Not};

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub struct Literal {
    literal_number: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Literal {
    pub fn new(variable: Variable, is_negated: bool) -> Self {
        Self {
            literal_number: variable.get_number() + variable.get_number() + (is_negated as i32),
        }
    }

    pub fn get_number(&self) -> i32 {
        self.literal_number >> 1
    }

    pub fn is_negated(&self) -> bool {
        (self.literal_number % 2) == 1
    }

    pub fn to_dimacs_literal(&self) -> String {
        let dimacs_number;
        if self.is_negated() {
            dimacs_number = -self.get_number();
        } else {
            dimacs_number = self.get_number();
        }
        dimacs_number.to_string()
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

impl Not for Literal {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Output {
            literal_number: self.literal_number | 1,
        }
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_negated() {
            write!(f, "!x{}", self.get_number())
        } else {
            write!(f, "x{}", self.get_number())
        }
    }
}
