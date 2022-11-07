// ************************************************************************************************
// use
// ************************************************************************************************

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    formulas::literal::VariableType,
    models::FiniteStateTransitionSystem,
    solvers::sat::{SatResponse, SplrSolver},
};

// ************************************************************************************************
// enum
// ************************************************************************************************

pub enum BMCResult {
    NoCTX {
        depth_reached: i32,
    },
    CTX {
        assignment: HashMap<VariableType, bool>,
        depth: VariableType,
    },
}

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct BMC {
    verbose: bool,
    solver: SplrSolver,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl BMC {
    // ************************************************************************************************
    // helper functions
    // ************************************************************************************************

    // ************************************************************************************************
    // api functions
    // ************************************************************************************************

    pub fn new(verbose: bool) -> Self {
        Self {
            solver: SplrSolver::default(),
            verbose,
        }
    }

    pub fn search(
        &self,
        fin_state: &FiniteStateTransitionSystem,
        search_depth_limit: u32,
        timeout_in_seconds: u64,
    ) -> BMCResult {
        let start = Instant::now();
        for depth in 0..(search_depth_limit + 1) {
            if self.verbose {
                println!(
                    "BMC running - depth = {}, elapsed time = {}",
                    depth,
                    start.elapsed().as_secs_f32()
                );
            }
            let elapsed_time = start.elapsed();
            if elapsed_time > Duration::from_secs(timeout_in_seconds) {
                let depth: i32 = depth.try_into().unwrap();
                return BMCResult::NoCTX {
                    depth_reached: (depth - 1),
                };
            }

            let mut sat_formula = fin_state.get_initial_relation();
            for unroll_depth in 1..(depth + 1) {
                sat_formula.append(&fin_state.get_transition_relation_for_some_depth(unroll_depth));
            }
            sat_formula.append(&fin_state.get_unsafety_property_for_some_depth(depth));

            let response = self.solver.solve_cnf(&sat_formula);
            match response {
                SatResponse::Sat { assignment } => return BMCResult::CTX { assignment, depth },
                SatResponse::UnSat => {}
            }
        }
        BMCResult::NoCTX {
            depth_reached: search_depth_limit.try_into().unwrap(),
        }
    }
}
