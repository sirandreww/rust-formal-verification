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
        depth_reached: VariableType,
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

    pub fn new() -> Self {
        Self {
            solver: SplrSolver::default(),
        }
    }

    pub fn search(
        &self,
        timeout_in_seconds: u64,
        fin_state: &FiniteStateTransitionSystem,
    ) -> BMCResult {
        let start = Instant::now();
        for depth in 0.. {
            let elapsed_time = start.elapsed();
            if elapsed_time > Duration::from_secs(timeout_in_seconds) {
                return BMCResult::NoCTX {
                    depth_reached: depth,
                };
            }

            let mut sat_formula = fin_state.get_initial_relation();
            for unroll_depth in 1..(depth + 1) {
                sat_formula.append(&fin_state.get_transition_relation_for_some_depth(unroll_depth));
            }
            sat_formula.append(&fin_state.get_unsafety_property_for_some_depth(depth));

            let response = self.solver.solve_cnf(&sat_formula);
            match response {
                SatResponse::Sat { assignment } => {
                    return BMCResult::CTX {
                        assignment,
                        depth,
                    }
                }
                SatResponse::UnSat => {}
            }
        }
        unreachable!();
    }
}

// ************************************************************************************************
// negation
// ************************************************************************************************

// ************************************************************************************************
// printing
// ************************************************************************************************
