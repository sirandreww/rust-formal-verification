// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod assignment;
pub mod cadical_solver;
pub mod sat_response;
pub mod splr_solver;
pub mod varisat_solver;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

use crate::formulas::CNF;
pub use assignment::Assignment;
pub use cadical_solver::CadicalSolver;
pub use sat_response::SatResponse;
pub use splr_solver::SplrSolver;
pub use varisat_solver::VarisatSolver;

// ************************************************************************************************
// Sat Solver trait
// ************************************************************************************************

pub trait SatSolver: Default {
    fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse;
}
