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
    true_variables: HashSet<VariableType>,
    false_variables: HashSet<VariableType>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Assignment {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn new(true_variables: HashSet<VariableType>, false_variables: HashSet<VariableType>) -> Self {
        Self {
            true_variables,
            false_variables,
        }
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

        Self::new(true_variables, false_variables)
    }

    pub fn get_value(&self, variable: &VariableType) -> Option<bool> {
        if self.true_variables.contains(variable) {
            Some(true)
        } else if self.false_variables.contains(variable) {
            Some(false)
        } else {
            None
        }
    }
}
