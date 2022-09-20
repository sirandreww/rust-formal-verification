// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use crate::solvers::sat::SatResponse;
use splr;

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
        let v = cnf_to_solve.to_vector_of_vectors();

        match splr::Certificate::try_from(v).expect("panic!") {
            splr::Certificate::UNSAT => SatResponse::UnSat {},
            splr::Certificate::SAT(assignment) => SatResponse::Sat { assignment },
        }
    }
}
