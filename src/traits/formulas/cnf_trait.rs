// ************************************************************************************************
// use
// ************************************************************************************************

use crate::traits::formulas::{ClauseTrait, LiteralTrait, VariableTrait};
use num_traits::int::PrimInt;

// ************************************************************************************************
// CNFTrait
// ************************************************************************************************

pub trait CNFTrait<
    IntType: PrimInt,
    Variable: VariableTrait<IntType>,
    Literal: LiteralTrait<IntType, Variable>,
    Clause: ClauseTrait<IntType, Variable, Literal>,
>
{
    fn add_clause(&mut self, new_clause: &Clause);
    fn to_dimacs(&self) -> String;
}
