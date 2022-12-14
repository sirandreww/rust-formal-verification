//! some algorithms that are already implemented.

// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod bmc; // requires file in this directory with the name 'bmc.rs'
pub mod formula_logic;
pub mod proof;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use bmc::BMC;
