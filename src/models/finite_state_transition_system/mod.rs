// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::{literal::VariableType, Clause, CNF};

// ************************************************************************************************
// struct
// ************************************************************************************************

/// Struct that describes memory layout of the finite state transition system.
///
/// implementations of many additional features can be found in sub-modules.
#[derive(Clone)]
pub struct FiniteStateTransitionSystem {
    initial_states: CNF,
    transition: CNF,
    state_and_property_connection: CNF,
    unsafety_property: Clause,
    max_literal_number: VariableType,
    state_literals: Vec<VariableType>,
    input_literals: Vec<VariableType>,
}

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod construction;
pub mod features;
pub mod getting;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************
