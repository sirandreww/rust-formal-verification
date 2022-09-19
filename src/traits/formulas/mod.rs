// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod clause_trait; // requires file in this directory with the name 'clause_trait.rs'
pub mod cnf_trait; // requires file in this directory with the name 'cnf_trait.rs'
pub mod literal_trait; // requires file in this directory with the name 'literal_trait.rs'
pub mod variable_trait; // requires file in this directory with the name 'variable_trait.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use clause_trait::ClauseTrait;
pub use cnf_trait::CNFTrait;
pub use literal_trait::LiteralTrait;
pub use variable_trait::VariableTrait;
