//! models like AndInverterGraph and FiniteStateTransitionSystem.

// ************************************************************************************************
// rust submodule decleration, they get searched in their respective file  names
// ************************************************************************************************

pub mod finite_state_transition_system;
// requires folder in this directory with the name 'finite_state_transition_system'
pub mod and_inverter_graph;
// requires folder in this directory with the name 'and_inverter_graph'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use and_inverter_graph::AndInverterGraph;
pub use finite_state_transition_system::FiniteStateTransitionSystem;
