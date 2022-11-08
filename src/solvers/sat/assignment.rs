// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::literal::VariableType;
use std::collections::HashSet;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    set_of_variables :HashSet<VariableType>,
    in_set_value: bool,
    out_of_set_value: bool,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Assignment {

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn new(set_of_variables : HashSet<VariableType>, in_set_value: bool) -> Self {
        Self { set_of_variables, in_set_value, out_of_set_value: !in_set_value }
    }

    // ********************************************************************************************
    // API
    // ********************************************************************************************

    pub fn from_dimacs_assignment(vector: &[i32]) -> Self {
        let mut true_variables = HashSet::<VariableType>::new();
        let mut false_variables = HashSet::<VariableType>::new();

        for var in vector.iter() {
            let var_num: VariableType = var.abs().try_into().unwrap();
            debug_assert!(var_num != 0);
            if var < &0 {
                false_variables.insert(var_num);
            } else {
                true_variables.insert(var_num);
            }
        }
        if true_variables.len() > false_variables.len() {
            // there are more true variables than false variables.
            // better to save the false variables.
            Self::new(false_variables, false)
        } else {
            // there are more false variables than true variables.
            // better to save the true variables.
            Self::new(true_variables, true)
        }
    }

    pub fn get_value_of_variable(&self, variable: &VariableType) -> bool {
        if self.set_of_variables.contains(variable){
            self.in_set_value
        } else {
            self.out_of_set_value
        }
    }
}
