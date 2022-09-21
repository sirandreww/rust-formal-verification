// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use std::fs;

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct FiniteStateTransitionSystem {
    _initial_states: CNF,
    _transition: CNF,
    _safety_property: CNF,
    _number_of_variables: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
    pub fn from_aig_string(aig_string: &str) -> FiniteStateTransitionSystem {
        let _vec_of_aig_lines = aig_string.split('\n');
        let initial_states = CNF::default();
        let transition = CNF::default();
        let safety_property = CNF::default();
        let number_of_variables = 0;
        FiniteStateTransitionSystem::new(
            initial_states,
            transition,
            safety_property,
            number_of_variables,
        )
    }

    pub fn from_aig_path(file_path: &String) -> FiniteStateTransitionSystem {
        let contents = fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("Unable to read the file '{file_path}'"));
        Self::from_aig_string(&contents)
    }

    pub fn new(
        initial_states: CNF,
        transition: CNF,
        safety_property: CNF,
        number_of_variables: i32,
    ) -> Self {
        debug_assert!(
            initial_states.get_greatest_variable_number() <= number_of_variables,
            "initial_states has variable with a higher number than number_of_variables."
        );
        debug_assert!(
            transition.get_greatest_variable_number() <= number_of_variables,
            "transition has variable with a higher number than number_of_variables."
        );
        Self {
            _initial_states: initial_states,
            _transition: transition,
            _safety_property: safety_property,
            _number_of_variables: number_of_variables,
        }
    }

    // pub fn Transition(&self) -> CNF {}
}
