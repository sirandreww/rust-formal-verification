//! Object for holding a finite state transition system, and performing operations like adding
//! tags and so on.

// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::{literal::VariableType, Clause, Cube, Literal, CNF};
use std::collections::{HashMap, HashSet};

// ************************************************************************************************
// struct
// ************************************************************************************************

/// Struct that describes memory layout of the finite state transition system.
///
/// implementations of many additional features can be found in sub-modules.
#[derive(Clone)]
pub struct FiniteStateTransitionSystem {
    initial_literals: HashSet<Literal>,
    initial_states: Cube,
    transition: CNF,
    state_to_safety_translation: CNF,
    unsafety_property: Clause,
    max_literal_number: VariableType,
    state_literals: Vec<VariableType>,
    input_literals: Vec<VariableType>,
    cone_of_safety: HashSet<VariableType>,
    cones_of_state_literals: HashMap<VariableType, HashSet<VariableType>>,
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
