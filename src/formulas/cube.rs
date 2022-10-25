// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Literal;
use crate::formulas::Clause;
use std::fmt;
use std::hash::Hash;
use std::ops::Not;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Cube {
    clause: Clause,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Cube {
    pub fn new(literals: &[Literal]) -> Self {
        Self { clause : Clause::new(literals) }
    }

    pub fn is_empty(&self) -> bool {
        self.clause.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.clause.iter()
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

impl Not for Cube {
    type Output = Clause;

    fn not(self) -> Self::Output {
        let mut literals = Vec::new();
        for lit in self.iter() {
            literals.push(!lit.to_owned());

        }
        Clause::new(&literals)
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_vec = self
            .clause
            .iter()
            .map(|lit| lit.to_string())
            .collect::<Vec<String>>();
        write!(f, "({})", string_vec.join(" & "))
    }
}

// impl IntoIterator for Clause {
//     type Item = Literal;
//     type IntoIter = <Vec<Literal> as IntoIterator>::IntoIter; // so that you don't have to write std::vec::IntoIter, which nobody remembers anyway

//     fn into_iter(self) -> Self::IntoIter {
//         self.literals.into_iter()
//     }
// }

//   // We deref to slice so that we can reuse the slice impls
//   impl Deref for BinaryVec {
//     type Output = [u8];

//     fn deref(&self) -> &[u8] {
//       &self.vec[..]
//     }
//   }
