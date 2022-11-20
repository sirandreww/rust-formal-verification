// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::literal::VariableType;
use crate::formulas::{Clause, Literal, CNF};

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
}
