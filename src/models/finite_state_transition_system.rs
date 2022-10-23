// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::{Clause, Literal, Variable, CNF};
use crate::models::AndInverterGraph;

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct FiniteStateTransitionSystem {
    _initial_states: CNF,
    _transition: CNF,
    _safety_property: CNF,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn new(initial_states: CNF, transition: CNF, safety_property: CNF) -> Self {
        // debug_assert!(
        //     initial_states.get_greatest_variable_number() <= number_of_variables,
        //     "initial_states has variable with a higher number than number_of_variables."
        // );
        // debug_assert!(
        //     transition.get_greatest_variable_number() <= number_of_variables,
        //     "transition has variable with a higher number than number_of_variables."
        // );
        Self {
            _initial_states: initial_states,
            _transition: transition,
            _safety_property: safety_property,
        }
    }

    fn create_initial_cnf(aig: &AndInverterGraph) -> CNF {
        let latch_info = aig.get_latch_information();
        let mut cnf = CNF::default();
        for (latch_literal, _, latch_reset) in latch_info {
            // if latch is initialized
            if latch_reset != (latch_literal >> 1) {
                let var_num: i32 = (latch_literal >> 1).try_into().unwrap();
                let lit = Literal::new(&Variable::new(var_num), false);
                if latch_reset == 0 {
                    cnf.add_clause(&Clause::new(&[!lit]));
                } else if latch_reset == 1 {
                    cnf.add_clause(&Clause::new(&[lit]));
                } else {
                    unreachable!();
                }
            }
        }
        cnf
    }

    // ********************************************************************************************
    // aig api functions
    // ********************************************************************************************

    /// Function that converts an AndInverterGraph into a FiniteStateTransitionSystem.
    ///
    /// # Arguments
    ///
    /// * `aig: &AndInverterGraph` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
    /// let file_path = "tests/simple_examples/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    /// assert_eq!(fsts.get_initial_states().to_string(), "((!x1) & (!x2) & (!x3))");
    /// ```
    pub fn from_aig(aig: &AndInverterGraph) -> Self {
        let initial_states: CNF = Self::create_initial_cnf(aig);
        let transition: CNF = CNF::default();
        let safety_property: CNF = CNF::default();
        Self::new(initial_states, transition, safety_property)
    }

    pub fn get_initial_states(&self) -> &CNF {
        &self._initial_states
    }

    // pub fn Transition(&self) -> CNF {}
}
