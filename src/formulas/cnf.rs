// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Clause;
use crate::formulas::Variable;
use std::cmp::max;
use std::collections::HashSet;
use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default)]
pub struct CNF {
    clauses: HashSet<Clause>,
    max_variable_number: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl CNF {
    pub fn add_clause(&mut self, new_clause: &Clause) {
        self.max_variable_number = max(
            self.max_variable_number,
            new_clause.get_highest_variable_number(),
        );
        self.clauses.insert((*new_clause).to_owned());
    }

    pub fn get_number_of_clauses(&self) -> usize {
        self.clauses.len()
    }

    pub fn get_new_variable(&mut self) -> Variable {
        self.max_variable_number += 1;
        Variable::new(self.max_variable_number)
    }

    pub fn get_highest_variable_number(&self) -> i32 {
        self.max_variable_number
    }

    /// Returns a String representing the CNF formula in dimacs format.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::CNF;
    /// use rust_formal_verification::formulas::Clause;
    /// use rust_formal_verification::formulas::Literal;
    ///
    /// let mut cnf = CNF::default();
    ///
    /// let l1 = Literal::new(cnf.get_new_variable(), false);
    /// let l2 = Literal::new(cnf.get_new_variable(), false);
    /// let l3 = Literal::new(cnf.get_new_variable(), false);
    ///
    /// cnf.add_clause(&Clause::new(vec![l1, l2, l3]));
    /// cnf.add_clause(&Clause::new(vec![!l1, l2, l3]));
    /// cnf.add_clause(&Clause::new(vec![l1, !l2, l3]));
    /// cnf.add_clause(&Clause::new(vec![l1, l2, !l3]));
    ///
    /// let dmacs_string = cnf.to_dimacs();
    /// let mut dmacs_vector = dmacs_string.split("\n").collect::<Vec<&str>>();
    /// dmacs_vector.sort();
    /// dmacs_vector.reverse();
    /// assert_eq!("p cnf 3 4\n1 2 3 0\n1 2 -3 0\n1 -2 3 0\n-1 2 3 0", dmacs_vector.join("\n"));
    /// ```
    pub fn to_dimacs(&self) -> String {
        let string_vec = self
            .clauses
            .iter()
            .map(|one_clause| one_clause.to_dimacs_line())
            .collect::<Vec<String>>();
        let dimacs_first_line = format!(
            "p cnf {} {}",
            self.max_variable_number,
            self.get_number_of_clauses()
        );
        let string_without_first_line = string_vec.join("\n");
        format!("{dimacs_first_line}\n{string_without_first_line}")
    }
}

// ************************************************************************************************
// default constructor
// ************************************************************************************************

// impl Default for CNF {
//     fn default() -> Self {
//         Self {
//             clauses: Default::default(),
//             max_variable_number: 0,
//         }
//     }
// }

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_vec = self
            .clauses
            .iter()
            .map(|one_clause| one_clause.to_string())
            .collect::<Vec<String>>();
        string_vec.sort();
        write!(f, "({})", string_vec.join(" & "))
    }
}
