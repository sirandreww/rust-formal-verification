// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod sat_response;
pub mod splr_solver;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use sat_response::SatResponse;
pub use splr_solver::SplrSolver;
