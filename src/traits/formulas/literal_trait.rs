// ************************************************************************************************
// use
// ************************************************************************************************

use crate::traits::formulas::VariableTrait;
use num_traits::int::PrimInt;

// ************************************************************************************************
// LiteralTrait
// ************************************************************************************************

pub trait LiteralTrait<IntType: PrimInt, Variable: VariableTrait<IntType>> {
    fn new(variable: Variable, is_negated: bool) -> Self;
    fn get_number(&self) -> IntType;
    fn is_negated(&self) -> bool;
}
