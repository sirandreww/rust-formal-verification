// ************************************************************************************************
// use
// ************************************************************************************************

use crate::{
    formulas::CNF,
    models::FiniteStateTransitionSystem,
    solvers::sat::{SatResponse, SatSolver},
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
/// use rust_formal_verification::solvers::sat::VarisatSolver;
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
pub fn does_a_imply_b<T: SatSolver>(a: &CNF, b: &CNF) -> bool {
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

pub fn is_a_and_b_satisfiable<T: SatSolver>(a: &CNF, b: &CNF) -> bool {
    let mut cnf_to_solve = a.to_owned();
    cnf_to_solve.append(b);
    let solver = T::default();
    match solver.solve_cnf(&cnf_to_solve) {
        SatResponse::Sat { assignment: _ } => {
            true
        }
        SatResponse::UnSat => {
            false
        }
    }
}

pub fn check_invariant<T: SatSolver>(fin_state: &FiniteStateTransitionSystem, inv_candidate: &CNF) {
    // println!("inv_candidate = {}", inv_candidate);
    // check INIT -> inv_candidate
    let mut init = fin_state.get_initial_relation();
    init.append(&fin_state.get_state_to_properties_relation());
    // println!("init = {}", init);

    assert!(
        does_a_imply_b::<T>(&init, inv_candidate),
        "Invariant does not cover all of init."
    );

    // check inv_candidate && Tr -> inv_candidate'
    let mut a = fin_state.get_transition_relation();
    a.append(inv_candidate);
    a.append(&fin_state.get_state_to_properties_relation());
    a.append(&fin_state.add_tags_to_relation(&fin_state.get_state_to_properties_relation(), 1));
    let b = fin_state.add_tags_to_relation(inv_candidate, 1);
    assert!(
        does_a_imply_b::<T>(&a, &b),
        "Invariant doesn't cover all of the reachable states."
    );

    // check inv_candidate ^ !p is un-sat
    let mut bad = fin_state.get_unsafety_property();
    bad.append(&fin_state.get_state_to_properties_relation());
    assert!(
        !is_a_and_b_satisfiable::<T>(inv_candidate, &bad),
        "Invariant isn't always safe.",
    );
}
