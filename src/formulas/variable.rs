// ************************************************************************************************
// use
// ************************************************************************************************

use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Variable {
    variable_number: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Variable {
    pub fn new(variable_number: i32) -> Self {
        assert!(
            variable_number > 0,
            "Variable number must be strictly positive."
        );
        Self { variable_number }
    }

    pub fn get_number(&self) -> i32 {
        self.variable_number
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "x{}", self.variable_number);
    }
}
