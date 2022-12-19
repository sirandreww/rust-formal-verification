// ************************************************************************************************
// use
// ************************************************************************************************

use crate::algorithms::formula_logic::{does_a_imply_b, is_a_and_b_satisfiable};
use crate::formulas::literal::VariableType;
use crate::formulas::{Clause, Literal, CNF};
use crate::solvers::sat::StatelessSatSolver;

use super::FiniteStateTransitionSystem;

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn bump_all_cnf_variables_by_some_number(
        original_cnf: &CNF,
        number_to_bump: VariableType,
    ) -> CNF {
        if number_to_bump == 0 {
            // this makes the function faster for the simple case
            original_cnf.to_owned()
        } else {
            let mut cnf_to_add_to = CNF::new();
            for clause in original_cnf.iter() {
                let mut literals = Vec::new();
                for literal in clause.iter() {
                    assert_ne!(literal.get_number(), 0);
                    let mut new_number = literal.get_number();
                    new_number += number_to_bump;
                    let is_negated = literal.is_negated();
                    let new_lit = Literal::new(new_number).negate_if_true(is_negated);
                    literals.push(new_lit);
                }

                let new_clause = Clause::new(&literals);
                cnf_to_add_to.add_clause(&new_clause);
            }
            cnf_to_add_to
        }
    }

    // ********************************************************************************************
    // aig api functions
    // ********************************************************************************************

    pub fn add_tags_to_relation(&self, relation: &CNF, number_of_tags: VariableType) -> CNF {
        Self::bump_all_cnf_variables_by_some_number(
            relation,
            self.max_literal_number * number_of_tags,
        )
    }

    pub fn check_invariant<T: StatelessSatSolver>(&self, inv_candidate: &CNF) {
        // println!("inv_candidate = {}", inv_candidate);
        // check INIT -> inv_candidate
        let mut init = self.get_initial_relation().to_cnf();
        init.append(&self.get_state_to_safety_translation());
        // println!("init = {}", init);

        assert!(
            does_a_imply_b::<T>(&init, inv_candidate),
            "Invariant does not cover all of init."
        );

        // check inv_candidate && Tr -> inv_candidate'
        let mut a = self.get_transition_relation();
        a.append(inv_candidate);
        a.append(&self.get_state_to_safety_translation());
        a.append(&self.add_tags_to_relation(&self.get_state_to_safety_translation(), 1));
        let b = self.add_tags_to_relation(inv_candidate, 1);
        assert!(
            does_a_imply_b::<T>(&a, &b),
            "Invariant doesn't cover all of the reachable states."
        );

        // check inv_candidate ^ !p is un-sat
        let mut bad = self.get_unsafety_property();
        bad.append(&self.get_state_to_safety_translation());
        assert!(
            !is_a_and_b_satisfiable::<T>(inv_candidate, &bad),
            "Invariant isn't always safe.",
        );
    }
}
