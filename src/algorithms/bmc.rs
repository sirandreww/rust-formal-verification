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
    solvers::sat::{Assignment, SatResponse, SatSolver},
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

pub struct BMC<T: SatSolver> {
    verbose: bool,
    solver: T,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl<T: SatSolver + std::marker::Send + 'static + std::marker::Sync + Clone> BMC<T> {
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
            solver: T::default(),
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
        // I
        let initial = fin_state.get_initial_relation();
        // println!("initial = {}", initial);
        // !P
        let mut not_p0 = fin_state.get_unsafety_property();
        not_p0.append(&fin_state.get_state_to_safety_translation());
        // println!("not_p0 = {}", not_p0);
        // Tr
        let tr = fin_state.get_transition_relation();
        // println!("tr = {}", tr);

        // loop for wanted depth
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

            let mut sat_formula = initial.to_owned();
            for unroll_depth in 0..depth {
                sat_formula.append(&fin_state.add_tags_to_relation(&tr, unroll_depth));
            }
            sat_formula.append(&fin_state.add_tags_to_relation(&not_p0, depth));

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
