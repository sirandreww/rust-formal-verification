// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod formulas; // requires existence of 'formulas/mod.rs'
pub mod models; // requires existence of 'models/mod.rs'
pub mod solvers; // requires existence of 'solvers/mod.rs'
                 // pub mod traits; // requires existence of 'traits/mod.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************
