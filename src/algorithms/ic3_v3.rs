// //! This algorithm is a rust implementation of the code that is in https://github.com/arbrad/IC3ref

// // ************************************************************************************************
// // use
// // ************************************************************************************************

// use crate::{
//     formulas::{literal::VariableType, Clause, Cube, Literal, CNF},
//     models::FiniteStateTransitionSystem,
//     solvers::sat::{Assignment, SatResponse, StatelessSatSolver, stateful::StatefulSatSolver},
// };
// use priority_queue::PriorityQueue;
// use rand::rngs::ThreadRng;
// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use std::{cmp::{max, Reverse}, collections::HashSet};
// use std::time;

// // ************************************************************************************************
// // Enum
// // ************************************************************************************************

// pub enum IC3V2Result {
//     Proof { invariant: CNF },
//     CTX { depth: VariableType },
// }

// struct Frame<T> {
//     k: usize,             // steps from initial state
//     borderCubes: HashSet<Cube>,  // additional cubes in this and previous frames
//     consecution: T,
// }

// // ************************************************************************************************
// // struct
// // ************************************************************************************************

// pub struct IC3V2<T> {
//     // for the algorithm
//     k: usize,
//     frames: Vec<Frame<T>>,
//     // clauses: Vec<CNF>,
//     // fin_state: FiniteStateTransitionSystem,
//     solver: T,
//     // rng: ThreadRng,
//     // latch_literals: Vec<u32>,
//     // _input_literals: Vec<u32>,

//     // caching for speedup
//     initial: CNF,
//     transition: CNF,
//     p0: CNF,
//     not_p0: CNF,
//     not_p1: CNF,

//     // for printing
//     verbose: bool,
//     number_of_sat_calls: u32,
//     time_in_sat_calls: time::Duration,
//     start_time: time::Instant,
// }

// // ************************************************************************************************
// // impl
// // ************************************************************************************************

// impl<T: StatefulSatSolver> IC3V2<T> {
//     // ********************************************************************************************
//     // sat calls
//     // ********************************************************************************************

//     fn sat_call(&mut self, cnf_to_solve: &CNF) -> SatResponse{
//         self.number_of_sat_calls += 1;
//         let start_time = time::Instant::now();
//         let result = self.solver.solve_cnf(cnf_to_solve);
//         self.time_in_sat_calls += start_time.elapsed();
//         result
//     }

//     // ********************************************************************************************
//     // helper functions
//     // ********************************************************************************************

//     fn is_bad_reached_in_0_steps(&mut self) -> SatResponse {
//         let mut cnf = CNF::new();
//         cnf.append(&self.initial);
//         cnf.append(&self.not_p0);
//         // println!("I ^ !P = {}", cnf);
//         self.sat_call(&cnf)
//     }

//     fn is_bad_reached_in_1_steps(&mut self) -> SatResponse {
//         let mut cnf = CNF::new();
//         cnf.append(&self.initial);
//         cnf.append(&self.transition);
//         cnf.append(&self.not_p1);
//         // println!("I ^ T ^ !P' = {}", cnf);
//         self.sat_call(&cnf)
//     }

//     // ********************************************************************************************
//     // API functions
//     // ********************************************************************************************

//     pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
//         let mut p0 = fin_state.get_state_to_safety_translation();
//         p0.append(&fin_state.get_safety_property());

//         let mut not_p0 = fin_state.get_state_to_safety_translation();
//         not_p0.append(&fin_state.get_unsafety_property());

//         Self {
//             clauses: Vec::new(),
//             fin_state: fin_state.to_owned(),
//             solver: T::default(),
//             rng: thread_rng(),
//             initial: fin_state.get_initial_relation().to_cnf(),
//             transition: fin_state.get_transition_relation(),
//             p0,
//             not_p0: not_p0.to_owned(),
//             not_p1: fin_state.add_tags_to_relation(&not_p0, 1),
//             latch_literals: fin_state.get_state_literal_numbers(),
//             _input_literals: fin_state.get_input_literal_numbers(),
//             verbose,
//             number_of_sat_calls: 0,
//             time_in_sat_calls: time::Duration::from_secs(0),
//             start_time: time::Instant::now(),
//         }
//     }

//     fn extend(&self) {
//         while self.frames.size() < (self.k + 2){

//         }
//     }

//     pub fn prove(&mut self) -> IC3V2Result {
//         // update start time.
//         self.start_time = time::Instant::now();

//         let init_and_not_p = self.is_bad_reached_in_0_steps();
//         match init_and_not_p {
//             SatResponse::Sat { assignment: _ } => return IC3V2Result::CTX { depth: 0 },
//             SatResponse::UnSat => (),
//         }

//         let init_and_tr_and_not_p_tag = self.is_bad_reached_in_1_steps();
//         match init_and_tr_and_not_p_tag {
//             SatResponse::Sat { assignment: _ } => return IC3V2Result::CTX { depth: 1 },
//             SatResponse::UnSat => (),
//         }

//         loop {
//             if self.verbose {
//                 println!("Level {}", self.k);
//                 self.extend();
//             }
//         }
// }
