// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::Clause;
use std::{collections::HashSet, fmt};

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Clone)]
pub struct CNF {
    clauses: HashSet<Clause>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl CNF {
    pub fn new() -> Self {
        Self {
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
    /// assert_eq!(cnf1.to_string(), "((!x1 | x2 | x3) & (x1 | !x2 | x3) & (x1 | x2 | !x3) & (x1 | x2 | x3))");
    /// ```
    pub fn add_clause(&mut self, new_clause: &Clause) {
        // self.max_variable_number = max(
        //     self.max_variable_number,
        //     new_clause.get_highest_variable_number(),
        // );
        self.clauses.insert(new_clause.to_owned());
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
    /// use rust_formal_verification::formulas::CNF;
    /// use rust_formal_verification::formulas::Clause;
    /// use rust_formal_verification::formulas::Literal;
    /// use rust_formal_verification::formulas::Variable;
    ///
    /// let v1 = Variable::new(1);
    /// let v2 = Variable::new(2);
    /// let v3 = Variable::new(3);
    ///
    /// let l1 = Literal::new(&v1, false);
    /// let l2 = Literal::new(&v2, false);
    /// let l3 = Literal::new(&v3, false);
    ///
    /// let c1 = Clause::new(&vec![l1, l2, l3]);
    /// let c2 = Clause::new(&vec![!l1, l2, l3]);
    /// let c3 = Clause::new(&vec![l1, !l2, l3]);
    /// let c4 = Clause::new(&vec![l1, l2, !l3]);
    ///
    /// let mut cnf1 = CNF::default();
    /// assert_eq!(cnf1.len(), 0);
    /// cnf1.add_clause(&c1);
    /// assert_eq!(cnf1.len(), 1);
    /// cnf1.add_clause(&c2);
    /// assert_eq!(cnf1.len(), 2);
    /// cnf1.add_clause(&c3);
    /// assert_eq!(cnf1.len(), 3);
    /// cnf1.add_clause(&c4);
    /// assert_eq!(cnf1.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    // pub fn get_highest_variable_number(&self) -> u32 {
    //     self.max_variable_number
    // }

    pub fn iter(&self) -> impl Iterator<Item = &Clause> {
        self.clauses.iter()
    }

    pub fn append(&mut self, cnf: &CNF) {
        self.clauses.extend(cnf.clauses.to_owned());
        // self.max_variable_number = max(self.max_variable_number, cnf.max_variable_number);
    }
}

// ************************************************************************************************
// default
// ************************************************************************************************

impl Default for CNF {
    fn default() -> Self {
        Self::new()
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut clauses = Vec::from_iter(self.clauses.iter());
        clauses.sort();
        let mut string_vec = clauses
            .iter()
            .map(|one_clause| one_clause.to_string())
            .collect::<Vec<String>>();
        string_vec.sort();
        write!(f, "({})", string_vec.join(" & "))
    }
}
