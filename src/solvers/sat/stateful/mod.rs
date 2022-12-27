//! module that hold sat solvers that are mutable and hold some state.

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod cadical_solver;
pub mod minisat_solver;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use cadical_solver::CaDiCalSolver;
pub use minisat_solver::MiniSatSolver;

// ************************************************************************************************
// use
// ************************************************************************************************

use super::SatResponse;
use crate::formulas::{Clause, Cube, CNF};

// ************************************************************************************************
// Sat Solver trait
// ************************************************************************************************

pub trait StatefulSatSolver: Default {
    fn add_cnf(&mut self, cnf: &CNF);
    fn solve(
        &mut self,
        temporary_extra_cube: Option<&Cube>,
        temporary_extra_clause: Option<&Clause>,
    ) -> SatResponse;
}
