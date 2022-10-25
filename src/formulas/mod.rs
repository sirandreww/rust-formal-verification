// ************************************************************************************************
// rust submodule declaration, they get searched in their respective file  names
// ************************************************************************************************

pub mod clause; // requires file in this directory with the name 'clause.rs'
pub mod cube; // requires file in this directory with the name 'cube.rs'
pub mod cnf; // requires file in this directory with the name 'cnf.rs'
pub mod literal; // requires file in this directory with the name 'literal.rs'
pub mod variable; // requires file in this directory with the name 'variable.rs'

// ************************************************************************************************
// re-exports of structs in these modules to simplify paths for other imports
// ************************************************************************************************

pub use clause::Clause;
pub use cnf::CNF;
pub use literal::Literal;
pub use variable::Variable;
pub use cube::Cube;
