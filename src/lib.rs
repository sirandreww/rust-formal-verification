// ************************************************************************************************
// rust submodule decleration, they get searched in their respective file  names
// ************************************************************************************************

pub mod formulas; // requires existance of 'formulas/mod.rs'
pub mod models;
pub mod solvers; // requires existance of 'solvers/mod.rs' // requires existance of 'models/mod.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************
