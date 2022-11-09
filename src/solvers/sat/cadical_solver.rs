// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::CNF;
use crate::solvers::sat::SatResponse;

use super::{Assignment, SatSolver};
// use std::time;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Default, Clone, Copy)]
pub struct CadicalSolver {}

// ************************************************************************************************
// impl CadicalSolver
// ************************************************************************************************

impl CadicalSolver {
    fn convert_cnf_to_dimacs_into_solver(cnf_to_solve: &CNF) -> Vec<Vec<i32>> {
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
        let mut solver: cadical::Solver = Default::default();

        let dimacs_format = Self::convert_cnf_to_dimacs_into_solver(cnf_to_solve);
        dimacs_format
            .iter()
            .for_each(|clause| solver.add_clause(clause.iter().copied()));
        // let start_time = time::Instant::now();
        // println!("Sat solver call - start!");
        let sat_call_response = solver.solve();
        // println!("Sat solver call - end! Duration was {} seconds.", start_time.elapsed().as_secs_f32());
        match sat_call_response {
            Some(c) => match c {
                false => SatResponse::UnSat {},
                true => {
                    let dimacs_assignment = (1..(solver.max_variable() + 1))
                        .map(|var_num| match solver.value(var_num) {
                            Some(v) => {
                                if v {
                                    var_num
                                } else {
                                    -var_num
                                }
                            }
                            None => var_num, // doesn't matter which value
                        })
                        .collect::<Vec<i32>>();

                    SatResponse::Sat {
                        assignment: Assignment::from_dimacs_assignment(&dimacs_assignment),
                    }
                }
            },
            None => unreachable!(),
        }
    }
}

// ************************************************************************************************
// impl trait
// ************************************************************************************************

impl SatSolver for CadicalSolver {
    fn solve_cnf(&self, cnf_to_solve: &CNF) -> SatResponse {
        self.solve_cnf(cnf_to_solve)
    }
}
