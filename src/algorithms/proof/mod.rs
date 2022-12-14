//! Algorithms for proving the safety of a finite state transition system.

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod ic3_stateful_solver;
pub mod ic3_stateless_solver;
pub mod pdr;
pub mod proof_result;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use ic3_stateful_solver::IC3Stateful;
pub use ic3_stateless_solver::IC3Stateless;
pub use pdr::PDR;
pub use proof_result::ProofResult;

// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::FiniteStateTransitionSystem;

// ************************************************************************************************
// Sat Solver trait
// ************************************************************************************************

pub trait FiniteStateTransitionSystemProver {
    fn new(fin_state: &FiniteStateTransitionSystem) -> Self;
    fn prove(&mut self) -> ProofResult;
}
