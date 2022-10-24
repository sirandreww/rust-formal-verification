// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use crate::solvers::sat::SatResponse;
use splr::{self, SolverError};
use splr::solver::SolverResult;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default)]
pub struct SplrSolver {}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl SplrSolver {
    pub fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        let v: &Vec<Vec<i32>> = cnf_to_solve.to_vector_of_vectors();
        let owned: Vec<Vec<i32>> = v.to_owned();
        // println!("{:?}", owned);

        match splr::Certificate::try_from(owned) {
            SolverResult::Ok(c) => {
                match c {
                    splr::Certificate::UNSAT => SatResponse::UnSat {},
                    splr::Certificate::SAT(assignment) => SatResponse::Sat { assignment },
                }
            },
            SolverResult::Err(e) => {
                match e {
                    SolverError::EmptyClause => SatResponse::UnSat {},
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}
