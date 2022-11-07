// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::AIGNode;

// ************************************************************************************************
// struct
// ************************************************************************************************

/// Struct that describes memory layout of the AIG.
///
/// implementations of many additional features can be found in sub-modules.
pub struct AndInverterGraph {
    maximum_variable_index: usize,
    number_of_inputs: usize,
    number_of_latches: usize,
    number_of_outputs: usize,
    number_of_and_gates: usize,
    number_of_bad_state_constraints: usize,
    number_of_invariant_constraints: usize,
    number_of_justice_constraints: usize,
    number_of_fairness_constraints: usize,

    nodes: Vec<AIGNode>, /* [0..maxvar] */

    // these contain indexes that are in nodes that have these nodes.
    inputs: Vec<usize>,
    latches: Vec<usize>,
    ands: Vec<usize>,

    // these contain literals.
    outputs: Vec<usize>,
    bad: Vec<usize>,
    constraints: Vec<usize>,
}

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod construction;
pub mod conversion;
pub mod getting;
pub mod simulation;

mod aig_node;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use simulation::AIGSimulationResult;
