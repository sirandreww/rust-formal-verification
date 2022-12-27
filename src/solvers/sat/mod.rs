//! module that holds objects and traits that performing sat solving in various ways.

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod assignment;
pub mod sat_response;
pub mod stateful;
pub mod stateless;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use assignment::Assignment;
pub use sat_response::SatResponse;
pub use stateless::StatelessSatSolver;
