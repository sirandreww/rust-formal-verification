// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::literal::VariableType;
use crate::formulas::{Cube, CNF};

use super::FiniteStateTransitionSystem;

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
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

    pub fn get_initial_relation(&self) -> Cube {
        self.initial_states.to_owned()
    }

    /// Function that gets the transition relation for the FiniteStateTransitionSystem.
    ///
    /// # Arguments
    ///
    /// * `self: &FiniteStateTransitionSystem` - the FiniteStateTransitionSystem desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
    /// use rust_formal_verification::formulas::CNF;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig, false);
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

    pub fn get_state_to_safety_translation(&self) -> CNF {
        self.state_to_safety_translation.to_owned()
    }

    pub fn get_safety_property(&self) -> CNF {
        (!self.unsafety_property.to_owned()).to_cnf()
    }

    pub fn get_unsafety_property(&self) -> CNF {
        self.unsafety_property.to_cnf()
    }
}
