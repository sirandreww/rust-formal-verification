// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use crate::solvers::sat::SatResponse;
use splr::solver::SolverResult;
use splr::{self, SolverError};

use super::{super::Assignment, StatelessSatSolver};
// use std::time;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default, Clone, Copy)]
pub struct SplrSolver {}

// ************************************************************************************************
// impl SplrSolver
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

    pub fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        let owned = Self::convert_cnf_to_vector_of_vectors(cnf_to_solve);

        // let start_time = time::Instant::now();
        // println!("Sat solver call - start!");
        let sat_call_response = splr::Certificate::try_from(owned);
        // println!("Sat solver call - end! Duration was {} seconds.", start_time.elapsed().as_secs_f32());
        match sat_call_response {
            SolverResult::Ok(c) => match c {
                splr::Certificate::UNSAT => SatResponse::UnSat {},
                splr::Certificate::SAT(assignment) => SatResponse::Sat {
                    assignment: Assignment::from_dimacs_assignment(&assignment),
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

// ************************************************************************************************
// impl trait
// ************************************************************************************************

impl StatelessSatSolver for SplrSolver {
    fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        self.solve_cnf(cnf_to_solve)
    }
}
