// ************************************************************************************************
// use
// ************************************************************************************************

use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{
    formulas::{literal::VariableType, CNF},
    models::FiniteStateTransitionSystem,
    solvers::sat::{SatResponse, SplrSolver, Assignment},
};

// ************************************************************************************************
// enum
// ************************************************************************************************

pub enum BMCResult {
    NoCTX {
        depth_reached: i32,
    },
    CTX {
        assignment: Assignment,
        depth: VariableType,
    },
}

enum TimedSatResult {
    TimeOut,
    NoTimeOut { response: SatResponse },
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
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn timed_sat_call(
        &self,
        sat_formula: CNF,
        start_instant: &Instant,
        timeout_duration: Duration,
    ) -> TimedSatResult {
        let solver = self.solver.to_owned();
        let join_handle = thread::spawn(move || solver.solve_cnf(&sat_formula));

        // wait until sat call finished or timeout has passed
        let mut sleep_duration_in_millis = 1;
        while !join_handle.is_finished() && (start_instant.elapsed() < timeout_duration) {
            thread::sleep(Duration::from_millis(sleep_duration_in_millis));
            // exponential increase no more than 1 second.
            if sleep_duration_in_millis < 1000 {
                sleep_duration_in_millis *= 2;
            }
        }

        if join_handle.is_finished() {
            // the sat call has stopped
            let response = join_handle.join().unwrap();
            TimedSatResult::NoTimeOut { response }
        } else {
            // let thread run until completion
            TimedSatResult::TimeOut
        }
    }

    // ********************************************************************************************
    // api functions
    // ********************************************************************************************

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
        timeout_duration: Duration,
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
            if start.elapsed() > timeout_duration {
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

            let timed_response = self.timed_sat_call(sat_formula, &start, timeout_duration);
            match timed_response {
                TimedSatResult::NoTimeOut { response } => match response {
                    SatResponse::Sat { assignment } => return BMCResult::CTX { assignment, depth },
                    SatResponse::UnSat => {}
                },
                TimedSatResult::TimeOut => {
                    let depth: i32 = depth.try_into().unwrap();
                    return BMCResult::NoCTX {
                        depth_reached: (depth - 1),
                    };
                }
            }
        }
        BMCResult::NoCTX {
            depth_reached: search_depth_limit.try_into().unwrap(),
        }
    }
}
