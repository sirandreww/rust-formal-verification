// ************************************************************************************************
// use
// ************************************************************************************************

use std::{fmt, ops::Not};

// ************************************************************************************************
// type alias
// ************************************************************************************************

/// This is the type that the literal
pub type VariableType = u32;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct Literal {
    literal_number: VariableType,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Literal {
    pub fn new(number: VariableType) -> Self {
        debug_assert!(number > 0, "Literal number may not be zero.");
        debug_assert!(
            number <= (VariableType::MAX >> 1),
            "Literal number is too big."
        );
        Self {
            literal_number: (number << 1),
        }
    }

    pub fn negate_if_true(&self, is_negated: bool) -> Self {
        if is_negated {
            !self.to_owned()
        } else {
            self.to_owned()
        }
    }

    pub fn get_number(&self) -> VariableType {
        self.literal_number >> 1
    }

    pub fn is_negated(&self) -> bool {
        (self.literal_number % 2) == 1
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
            write!(f, "-{}", self.get_number())
        } else {
            write!(f, "{}", self.get_number())
        }
    }
}
