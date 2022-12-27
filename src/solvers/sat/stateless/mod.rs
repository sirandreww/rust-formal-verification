//! module that holds sat solvers that are not mutable and don't hold a state.

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod cadical_solver;
pub mod splr_solver;
pub mod varisat_solver;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use cadical_solver::CaDiCalSolver;
pub use splr_solver::SplrSolver;
pub use varisat_solver::VarisatSolver;

// ************************************************************************************************
// use
// ************************************************************************************************

use super::SatResponse;
use crate::formulas::CNF;

// ************************************************************************************************
// Sat Solver trait
// ************************************************************************************************

pub trait StatelessSatSolver: Default {
    fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse;
}
