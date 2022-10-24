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
    literal_number: u32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Literal {
    pub fn new(variable: &Variable) -> Self {
        Self {
            literal_number: variable.get_number() << 1,
        }
    }

    pub fn new_with_negation_option(variable: &Variable, is_negated: bool) -> Self {
        if is_negated {
            !Self::new(variable)
        } else {
            Self::new(variable)
        }
    }

    pub fn get_number(&self) -> u32 {
        self.literal_number >> 1
    }

    pub fn is_negated(&self) -> bool {
        (self.literal_number % 2) == 1
    }

    pub fn to_dimacs_number(&self) -> i32 {
        let lit_num = self.get_number();
        // this should succeed because lit num cannot be to large to overflow.
        let mut lit_num_as_signed_number: i32 = lit_num.try_into().unwrap();
        if self.is_negated() {
            lit_num_as_signed_number = -lit_num_as_signed_number;
        }
        lit_num_as_signed_number
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

impl Not for Literal {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Output {
            literal_number: self.literal_number ^ 1,
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
