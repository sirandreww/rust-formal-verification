// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod aiger;
// requires folder in this directory with the name 'aiger'
mod aiger_node;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use aiger::AndInverterGraph;
