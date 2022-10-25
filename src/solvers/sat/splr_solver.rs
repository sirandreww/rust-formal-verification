// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::{CNF, Cube, Variable, Literal};
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

    fn convert_assignment_to_cube(assignment: Vec<i32>) -> Cube {
        let mut literals = vec![];
        for number in assignment {
            let var = Variable::new(number.abs().try_into().unwrap());
            let lit = Literal::new(&var);
            if number < 0 {
                literals.push(!lit);
            } else {
                literals.push(lit);
            }
        }
        Cube::new(&literals)
    }


    pub fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        let v: &Vec<Vec<i32>> = cnf_to_solve.to_vector_of_vectors();
        let owned: Vec<Vec<i32>> = v.to_owned();
        // println!("{:?}", owned);

        match splr::Certificate::try_from(owned) {
            SolverResult::Ok(c) => {
                match c {
                    splr::Certificate::UNSAT => SatResponse::UnSat {},
                    splr::Certificate::SAT(assignment) => SatResponse::Sat { cube: Self::convert_assignment_to_cube(assignment) },
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
