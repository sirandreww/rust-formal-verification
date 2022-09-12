// ************************************************************************************************
// rust submodule decleration, they get searched in their respective file  names
// ************************************************************************************************

pub mod fsts; // requires file in this directory with the name 'fsts.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use fsts::FSTS;
