// ************************************************************************************************
// rust submodule decleration, they get searched in their respective file  names
// ************************************************************************************************

pub mod minisat;
pub mod sat_solver; // requires file in this directory with the name 'minisat.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use minisat::MinisatSolver;
pub use sat_solver::SatSolver;
