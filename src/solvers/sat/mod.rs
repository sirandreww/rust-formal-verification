// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod splr_solver;
pub mod sat_response;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use splr_solver::SplrSolver;
pub use sat_response::SatResponse;
