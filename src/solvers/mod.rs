// ************************************************************************************************
// rust submodule decleration, they get searched in their respective file  names
// ************************************************************************************************

pub mod z3_solver;  // requires file in this directory with the name 'z3_solver.rs'
pub mod sat_solver;
// mod solvers;        

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use z3_solver::Z3Solver;
pub use sat_solver::SatSolver;
