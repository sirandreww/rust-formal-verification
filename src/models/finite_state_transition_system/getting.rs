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

    pub fn get_max_literal_number(&self) -> VariableType {
        self.max_literal_number
    }

    pub fn get_state_literal_numbers(&self) -> Vec<VariableType> {
        self.state_literals.to_owned()
    }

    pub fn get_input_literal_numbers(&self) -> Vec<VariableType> {
        self.input_literals.to_owned()
    }

    pub fn get_initial_relation(&self) -> CNF {
        self.initial_states.to_owned()
    }

    /// Function that gets the transition relation for the FiniteStateTransitionSystem.
    ///
    /// # Arguments
    ///
    /// * `self: &FiniteStateTransitionSystem` - the FiniteStateTransitionSystem desired.
    /// * `depth: u32` - the number of tags desire, for example for Tr(X, X') this would be 1
    ///                  for Tr(X', X'') this would be 2 and so on.
    /// * `cnf_to_add_to: &mut CNF` - cnf that the result would be added to for performance reasons.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
    /// use rust_formal_verification::formulas::CNF;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    /// let mut tr_x_x_tag = CNF::default();
    /// let tr_x_x_tag = fsts.get_transition_relation();
    /// assert_eq!(
    ///     tr_x_x_tag.to_string(),
    ///     "p cnf 8 12\n1 -7 0\n-1 -5 0\n-1 7 0\n2 -8 0\n-2 -4 0\n-2 8 0\n-3 -4 0\n4 -5 0\n5 -6 0\n-5 6 0\n1 -4 5 0\n2 3 4 0"
    /// );
    /// ```
    pub fn get_transition_relation(&self) -> CNF {
        self.transition.to_owned()
    }

    pub fn get_state_to_properties_relation(&self) -> CNF {
        self.state_and_property_connection.to_owned()
    }

    pub fn get_safety_property(&self) -> CNF {
        (!self.unsafety_property.to_owned()).to_cnf()
    }

    pub fn get_unsafety_property(&self) -> CNF {
        self.unsafety_property.to_cnf()
    }

    pub fn add_tags_to_relation(&self, relation: &CNF, number_of_tags: VariableType) -> CNF {
        Self::bump_all_cnf_variables_by_some_number(
            relation,
            self.max_literal_number * number_of_tags,
        )
    }
}
