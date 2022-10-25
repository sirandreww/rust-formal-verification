// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use std::fmt;
use std::hash::Hash;
// use std::ops::Not;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct Clause {
    literals: Vec<Literal>,
    max_variable_number: u32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Clause {
    pub fn new(literals: &[Literal]) -> Self {
        Self {
            literals: literals.to_owned(),
            max_variable_number: literals.iter().map(|l| l.get_number()).max().unwrap_or(0),
        }
    }

    pub fn get_highest_variable_number(&self) -> u32 {
        self.max_variable_number
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

// impl Not for Clause {
//     type Output = Cube;

//     fn not(self) -> Self::Output {
//         let mut literals = Vec::new();
//         for lit in self.iter() {
//             literals.push(!lit.to_owned());

//         }
//         Cube::new(&literals)
//     }
// }

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
