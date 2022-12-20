// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod bmc; // requires file in this directory with the name 'bmc.rs'
pub mod formula_logic;
pub mod ic3;
pub mod ic3_v2;
pub mod ic3_v3;
pub mod pdr;

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use bmc::BMC;
pub use ic3::IC3;
pub use ic3_v2::IC3V2;
// pub use pdr::PDR;
