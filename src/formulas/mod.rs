// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod clause; // requires file in this directory with the name 'clause.rs'
pub mod cnf; // requires file in this directory with the name 'cnf.rs'
pub mod cube;
pub mod literal; // requires file in this directory with the name 'literal.rs' // requires file in this directory with the name 'cube.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use clause::Clause;
pub use cnf::CNF;
pub use cube::Cube;
pub use literal::Literal;
