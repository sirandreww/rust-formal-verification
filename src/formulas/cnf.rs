// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Clause;
use std::{cmp::max, collections::HashSet, fmt};

use super::literal::VariableType;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Clone)]
pub struct CNF {
    max_variable_number: VariableType,
    clauses: HashSet<Clause>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl CNF {
    pub fn new() -> Self {
        Self {
            max_variable_number: 0,
            clauses: HashSet::new(),
        }
    }

    /// Function that adds a clause to the CNF.
    /// If the clause already exists then it is not added.
    ///
    /// # Arguments
    ///
    /// * `self` - a mut reference to self.
    /// * `new_clause` - an immutable reference to a Clause you want to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::{CNF, Clause, Literal};
    /// let l1 = Literal::new(1);
    /// let l2 = Literal::new(2);
    /// let l3 = Literal::new(3);
    /// let mut cnf1 = CNF::default();
    /// cnf1.add_clause(&Clause::new(&vec![l1, l2, l3]));
    /// cnf1.add_clause(&Clause::new(&vec![!l1, l2, l3]));
    /// cnf1.add_clause(&Clause::new(&vec![l1, !l2, l3]));
    /// cnf1.add_clause(&Clause::new(&vec![l1, l2, !l3]));
    /// assert_eq!(cnf1.to_string(), "p cnf 3 4\n1 2 3 0\n1 2 -3 0\n1 -2 3 0\n-1 2 3 0");
    /// ```
    pub fn add_clause(&mut self, new_clause: &Clause) {
        self.max_variable_number = max(
            self.max_variable_number,
            new_clause.get_highest_variable_number(),
        );
        self.clauses.insert(new_clause.to_owned());
    }

    pub fn contains(&self, clause: &Clause) -> bool {
        self.clauses.contains(clause)
    }

    /// Function that returns the number of clauses that are currently in the CNF.
    ///
    /// # Arguments
    ///
    /// * `self` - an immutable reference to self.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::{CNF, Clause, Literal};
    /// let l1 = Literal::new(1);
    /// let l2 = Literal::new(2);
    /// let l3 = Literal::new(3);
    /// let mut cnf1 = CNF::default();
    /// assert_eq!(cnf1.len(), 0);
    /// cnf1.add_clause(&Clause::new(&[l1, l2, l3]));
    /// assert_eq!(cnf1.len(), 1);
    /// cnf1.add_clause(&Clause::new(&[!l1, l2, l3]));
    /// assert_eq!(cnf1.len(), 2);
    /// cnf1.add_clause(&Clause::new(&[l1, !l2, l3]));
    /// assert_eq!(cnf1.len(), 3);
    /// cnf1.add_clause(&Clause::new(&[l1, l2, !l3]));
    /// assert_eq!(cnf1.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    /// Function that returns if the cnf is empty or not.
    ///
    /// # Arguments
    ///
    /// * `self` - an immutable reference to self.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::{CNF, Clause, Literal};
    /// let l1 = Literal::new(1);
    /// let l2 = Literal::new(2);
    /// let l3 = Literal::new(3);
    /// let mut cnf1 = CNF::default();
    /// assert!(cnf1.is_empty());
    /// cnf1.add_clause(&Clause::new(&[l1, l2, l3]));
    /// assert!(!cnf1.is_empty());
    /// cnf1.add_clause(&Clause::new(&[!l1, l2, l3]));
    /// assert!(!cnf1.is_empty());
    /// cnf1.add_clause(&Clause::new(&[l1, !l2, l3]));
    /// assert!(!cnf1.is_empty());
    /// cnf1.add_clause(&Clause::new(&[l1, l2, !l3]));
    /// assert!(!cnf1.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    /// Function that returns an iterator over the clauses of the cnf.
    ///
    /// # Arguments
    ///
    /// * `self` - an immutable reference to self.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::{CNF, Clause, Literal};
    /// let l1 = Literal::new(1);
    /// let l2 = Literal::new(2);
    /// let l3 = Literal::new(3);
    /// let mut cnf1 = CNF::default();
    /// cnf1.add_clause(&Clause::new(&[l1, l2, l3]));
    /// cnf1.add_clause(&Clause::new(&[!l1, l2, l3]));
    /// cnf1.add_clause(&Clause::new(&[l1, !l2, l3]));
    /// cnf1.add_clause(&Clause::new(&[l1, l2, !l3]));
    /// for c in cnf1.iter() {
    ///     assert_eq!(c.len(), 3);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &Clause> {
        self.clauses.iter()
    }

    /// Function that appends another CNF to self.
    ///
    /// # Arguments
    ///
    /// * `self` - an immutable reference to self.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::{CNF, Clause, Literal};
    /// let l1 = Literal::new(1);
    /// let l2 = Literal::new(2);
    /// let l3 = Literal::new(3);
    /// let mut cnf1 = CNF::default();
    /// cnf1.add_clause(&Clause::new(&[l1, l2, l3]));
    /// cnf1.add_clause(&Clause::new(&[!l1, l2, l3]));
    ///
    /// let mut cnf2 = CNF::default();
    /// cnf2.add_clause(&Clause::new(&[l1, !l2, l3]));
    /// cnf2.add_clause(&Clause::new(&[l1, l2, !l3]));
    ///
    /// assert_eq!(cnf1.len(), 2);
    /// cnf1.append(&cnf2);
    /// assert_eq!(cnf1.len(), 4);
    /// ```
    pub fn append(&mut self, cnf: &CNF) {
        self.max_variable_number = max(self.max_variable_number, cnf.max_variable_number);
        self.clauses.extend(cnf.clauses.to_owned());
    }
}

// ************************************************************************************************
// Default
// ************************************************************************************************

impl Default for CNF {
    fn default() -> Self {
        Self::new()
    }
}

// ************************************************************************************************
// PartialEq
// ************************************************************************************************

impl PartialEq for CNF {
    fn eq(&self, other: &Self) -> bool {
        self.clauses == other.clauses
    }
}

// ************************************************************************************************
// Eq
// ************************************************************************************************

impl Eq for CNF {}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut clauses = Vec::from_iter(self.clauses.iter());
        clauses.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        let string_vec = clauses
            .iter()
            .map(|one_clause| one_clause.to_string())
            .collect::<Vec<String>>();
        // string_vec.sort();
        write!(
            f,
            "p cnf {} {}\n{}",
            self.max_variable_number,
            self.len(),
            string_vec.join("\n")
        )
    }
}
