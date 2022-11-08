// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use crate::solvers::sat::SatResponse;
use varisat::{ ExtendFormula, Solver, Lit};

use super::Assignment;
// use std::time;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default, Clone, Copy)]
pub struct VarisatSolver {}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl VarisatSolver {
    fn convert_cnf_to_varisat(cnf_to_solve: &CNF, solver_to_add_to: &mut Solver) {
        for clause in cnf_to_solve.iter() {
            let mut varisat_literals = Vec::new();
            for lit in clause.iter() {
                let number: isize = lit.get_number().try_into().unwrap();
                let signed_number = if lit.is_negated() { -number } else { number };
                varisat_literals.push(Lit::from_dimacs(signed_number));
            }
            solver_to_add_to.add_clause(&varisat_literals);
        }
    }

    fn varisat_model_to_dimacs_assignment(assignment: &Vec<varisat::Lit>) -> Vec<i32>{
        assignment.iter().map(|l| l.to_dimacs().try_into().unwrap()).collect::<Vec<i32>>()
    }


    pub fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        let mut solver = Solver::new();
        Self::convert_cnf_to_varisat(&cnf_to_solve, &mut solver);

        // let start_time = time::Instant::now();
        // println!("Sat solver call - start!");
        let sat_call_response = solver.solve();
        // println!("Sat solver call - end! Duration was {} seconds.", start_time.elapsed().as_secs_f32());
        match sat_call_response {
            Ok(is_sat) => match is_sat {
                true => {
                    SatResponse::Sat { 
                        assignment: Assignment::from_dimacs_assignment(
                            &Self::varisat_model_to_dimacs_assignment(
                                &solver.model().unwrap()
                            )
                        )
                    }
                },
                false => {
                    SatResponse::UnSat {}
                },
            },
            Err(_) => {
                panic!();
            },
            // SolverResult::Ok(c) => match c {
            //     splr::Certificate::UNSAT => SatResponse::UnSat {},
            //     splr::Certificate::SAT(assignment) => SatResponse::Sat {
            //         assignment: Assignment::from_dimacs_vector(&assignment),
            //     },
            // },
            // SolverResult::Err(e) => match e {
            //     SolverError::EmptyClause => SatResponse::UnSat {},
            //     _ => {
            //         unreachable!();
            //     }
            // },
        }
    }
}
