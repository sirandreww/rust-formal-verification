// ************************************************************************************************
// use
// ************************************************************************************************

use std::fmt;
use std::hash::Hash;
use crate::formulas::Variable;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Literal {
    varialble : Variable,
    is_negated : bool
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Literal {
    pub fn new(varialble: Variable, is_negated : bool) -> Self {
        Self { varialble, is_negated }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_negated {
            return write!(f, "!{}", self.varialble.to_string());
        } else {
            return write!(f, "{}", self.varialble.to_string());
        }
    }
}
