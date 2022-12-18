//! Utilities for the creation and use of bit-level symbolic model checking algorithms.
//!
//! rust_formal_verification provides utilities to read AIGs, to convert them to
//! useful types such as finite state transition formulas, and some common algorithms.
//! This crate is for developing and prototyping algorithms for formal verification for hardware
//! devices. Algorithms like BMC, IC3, PDR and so on...
//!
//! # Quick Start
//!
//! To get you started quickly, all you need to do is read an .aig file.
//!
//! ```
//! use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
//!
//! // read aig file:
//! let file_path = "tests/examples/ours/counter.aig";
//! let aig = AndInverterGraph::from_aig_path(file_path);
//!
//! // create boolean logic formulas that represent aig:
//! let fsts = FiniteStateTransitionSystem::from_aig(&aig, false);
//!
//! // the formulas can then be read and turned to strings in DIMACS format.
//! assert_eq!(fsts.get_initial_relation().to_string(), "p cnf 3 3\n-1 0\n-2 0\n-3 0");
//! ```

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

/// some algorithms that are already implemented.
pub mod algorithms;

/// representation of boolean logic formulas.
pub mod formulas; // requires existence of 'formulas/mod.rs'

/// models like AndInverterGraph and FiniteStateTransitionSystem.
pub mod models; // requires existence of 'models/mod.rs'

/// solvers for hard problems.
pub mod solvers; // requires existence of 'solvers/mod.rs' // requires existence of 'algorithms/mod.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************
