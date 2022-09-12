// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Variable;
use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
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
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.literal_number % 2) == 0 {
            return write!(f, "x{}", (self.get_number()));
        } else {
            return write!(f, "!x{}", (self.get_number()));
        }
    }
}
