// ************************************************************************************************
// use
// ************************************************************************************************

use crate::traits::formulas::{LiteralTrait, VariableTrait};
use num_traits::int::PrimInt;

// ************************************************************************************************
// ClauseTrait
// ************************************************************************************************

pub trait ClauseTrait<
    IntType: PrimInt,
    Variable: VariableTrait<IntType>,
    Literal: LiteralTrait<IntType, Variable>,
>
{
    fn new(literals: &[Literal]) -> Self;
    fn add_literal(&mut self, new_literal: &Literal);
    fn to_dimacs_line(&self) -> String;
}
