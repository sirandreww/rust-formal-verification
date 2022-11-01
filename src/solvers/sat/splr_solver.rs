// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use crate::formulas::literal::VariableType;
use crate::solvers::sat::SatResponse;
use splr::solver::SolverResult;
use splr::{self, SolverError};
use std::collections::HashMap;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default)]
pub struct SplrSolver {}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl SplrSolver {
    fn convert_cnf_to_vector_of_vectors(cnf_to_solve: &CNF) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for clause in cnf_to_solve.iter() {
            let mut i32_lits = Vec::new();
            for lit in clause.iter() {
                let number: i32 = lit.get_number().try_into().unwrap();
                let signed_number = if lit.is_negated() { -number } else { number };
                i32_lits.push(signed_number);
            }
            result.push(i32_lits);
        }
        result
    }

    fn convert_result_to_hashmap(assignment: &Vec<i32>) -> HashMap<VariableType, bool> {
        let mut result = HashMap::<VariableType, bool>::new();
        for var in assignment {
            let var_num: VariableType = var.abs().try_into().unwrap();
            debug_assert!(var_num != 0);
            result.insert(var_num, var > &0);
        }
        result
    }

    pub fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        let owned = Self::convert_cnf_to_vector_of_vectors(cnf_to_solve);

        match splr::Certificate::try_from(owned) {
            SolverResult::Ok(c) => match c {
                splr::Certificate::UNSAT => SatResponse::UnSat {},
                splr::Certificate::SAT(assignment) => SatResponse::Sat {
                    assignment: Self::convert_result_to_hashmap(&assignment),
                },
            },
            SolverResult::Err(e) => match e {
                SolverError::EmptyClause => SatResponse::UnSat {},
                _ => {
                    unreachable!();
                }
            },
        }
    }
}
