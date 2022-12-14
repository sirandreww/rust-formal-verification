// ************************************************************************************************
// use
// ************************************************************************************************

use std::collections::HashSet;

use crate::{
    formulas::{literal::VariableType, Clause, Literal, CNF},
    solvers::sat::{Assignment, SatResponse, StatelessSatSolver},
};

// ************************************************************************************************
// enum
// ************************************************************************************************

// ************************************************************************************************
// struct
// ************************************************************************************************

// ************************************************************************************************
// some functions
// ************************************************************************************************

/// Functions that returns true iff a -> b.
///
/// # Arguments
///
/// * `a` - CNF formula.
/// * `b` - CNF formula.
///
/// # Examples
///
/// ```
/// use rust_formal_verification::formulas::{CNF, Clause, Literal};
/// use rust_formal_verification::algorithms::formula_logic::does_a_imply_b;
/// use rust_formal_verification::solvers::sat::stateless::VarisatSolver;
/// let l1 = Literal::new(1);
/// let l2 = Literal::new(2);
/// let l3 = Literal::new(3);
/// let l4 = Literal::new(4);
/// let l5 = Literal::new(5);
/// let l6 = Literal::new(6);
///
/// let mut all_literals_are_equal = CNF::default();
/// all_literals_are_equal.add_clause(&Clause::new(&[l1, !l2]));
/// all_literals_are_equal.add_clause(&Clause::new(&[l2, !l3]));
/// all_literals_are_equal.add_clause(&Clause::new(&[l3, !l4]));
/// all_literals_are_equal.add_clause(&Clause::new(&[l4, !l5]));
/// all_literals_are_equal.add_clause(&Clause::new(&[l5, !l6]));
/// all_literals_are_equal.add_clause(&Clause::new(&[l6, !l1]));
///
/// let mut one_and_4_are_equal = CNF::default();
/// one_and_4_are_equal.add_clause(&Clause::new(&[l1, !l4]));
/// one_and_4_are_equal.add_clause(&Clause::new(&[l4, !l1]));
///
/// assert!(does_a_imply_b::<VarisatSolver>(&all_literals_are_equal, &one_and_4_are_equal));
/// ```
pub fn does_a_imply_b<T: StatelessSatSolver>(a: &CNF, b: &CNF) -> bool {
    // a implies b iff a implies every clause in b
    // println!("a = {}", a);
    // println!("b = {}", b);
    for c in b.iter() {
        let cube = !c.to_owned();
        let mut cnf_to_solve = cube.to_cnf();
        cnf_to_solve.append(a);
        let solver = T::default();
        match solver.solve_cnf(&cnf_to_solve) {
            SatResponse::Sat { assignment: _ } => {
                return false;
            }
            SatResponse::UnSat => {}
        }
    }
    true
}

pub fn is_a_and_b_satisfiable<T: StatelessSatSolver>(a: &CNF, b: &CNF) -> bool {
    let mut cnf_to_solve = a.to_owned();
    cnf_to_solve.append(b);
    let solver = T::default();
    match solver.solve_cnf(&cnf_to_solve) {
        SatResponse::Sat { assignment: _ } => true,
        SatResponse::UnSat => false,
    }
}

pub fn evaluate_assignment_on_literal(literal: &Literal, assignment: &Assignment) -> bool {
    let value = assignment.get_value(&literal.get_number()).unwrap();
    if literal.is_negated() {
        !value
    } else {
        value
    }
}

pub fn evaluate_assignment_on_clause(clause: &Clause, assignment: &Assignment) -> bool {
    for literal in clause.iter() {
        if evaluate_assignment_on_literal(literal, assignment) {
            return true;
        }
    }
    false
}

pub fn evaluate_assignment_on_cnf(cnf: &CNF, assignment: &Assignment) -> bool {
    for clause in cnf.iter() {
        if !evaluate_assignment_on_clause(clause, assignment) {
            return false;
        }
    }
    true
}

pub fn get_all_variable_numbers_in_cnf(cnf: &CNF) -> HashSet<VariableType> {
    let mut set = HashSet::new();
    for clause in cnf.iter() {
        for literal in clause.iter() {
            set.insert(literal.get_number());
        }
    }
    set
}
