// ************************************************************************************************
// use
// ************************************************************************************************

use std::fmt;
use std::hash::Hash;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Variable {
    variable_number : u32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Variable {
    pub fn new(variable_number: u32) -> Self {
        assert!(variable_number > 0, "Variable number must be strictly positive.");
        Self { variable_number: variable_number }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "x{}", self.variable_number);
    }
}
