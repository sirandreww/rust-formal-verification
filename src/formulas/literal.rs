// ************************************************************************************************
// use
// ************************************************************************************************

use std::fmt;
use std::hash::Hash;

// ************************************************************************************************
// struct
// ************************************************************************************************

/// struct that represents a literal. Can be either negated or not.
///
/// # Example
///
/// ```
/// // let l = rust_formal_verification::formulas::Literal::new(42, false);
/// ```
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Literal {
    varialble_number : u32,
    is_negated : bool
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Literal {
    /// struct that represents a literal. Can be either negated or not.
    ///
    /// # Example
    ///
    /// ```
    /// // let l = Literal::new(42, false);
    /// ```
    pub fn new(varialble_number: u32, is_negated : bool) -> Self {
        Self { varialble_number, is_negated }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_negated {
            return write!(f, "!x{}", self.varialble_number);
        } else {
            return write!(f, "x{}", self.varialble_number);
        }
        
    }
}
