// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod minisat_solver;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use minisat_solver::MiniSatSolver;

// ************************************************************************************************
// use
// ************************************************************************************************

use super::SatResponse;
use crate::formulas::{Cube, CNF};

// ************************************************************************************************
// Sat Solver trait
// ************************************************************************************************

pub trait StatefulSatSolver: Default {
    fn add_cnf(&mut self, cnf: &CNF);
    fn solve(&mut self) -> SatResponse;
    fn solve_under_assumptions(&mut self, assumptions: &Cube) -> SatResponse;
}
