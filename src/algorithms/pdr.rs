//! This algorithm is an exact implementation of what is described in "Efficient implementation of property directed reachability".
//!
//! N. Een, A. Mishchenko and R. Brayton,
//! "Efficient implementation of property directed reachability,"
//! 2011 Formal Methods in Computer-Aided Design (FMCAD), 2011, pp. 125-134.
//!
//! Abstract: Last spring, in March 2010, Aaron Bradley published the first truly new bit-level
//! symbolic model checking algorithm since Ken McMillan's interpolation based model checking
//! procedure introduced in 2003.
//! Our experience with the algorithm suggests that it is stronger than interpolation on industrial
//! problems, and that it is an important algorithm to study further.
//! In this paper, we present a simplified and faster implementation of Bradley's procedure, and
//! discuss our successful and unsuccessful attempts to improve it.
//! URL: https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=6148886&isnumber=6148882
//!
//! The implementation of the original 2010 bit-level symbolic model checking algorithm is
//! available under ic3.

// ************************************************************************************************
// use
// ************************************************************************************************

use crate::{
    formulas::{literal::VariableType, Clause, Cube, Literal, CNF},
    models::FiniteStateTransitionSystem,
    solvers::sat::{Assignment, SatResponse, SatSolver},
};
use priority_queue::PriorityQueue;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{max, Reverse};
use std::{cmp::min, collections::BinaryHeap};

// ************************************************************************************************
// Enum
// ************************************************************************************************

pub enum PDRResult {
    Proof { invariant: CNF },
    CTX { depth: VariableType },
}

pub enum Frame {
    NULL,
    INF,
    Ok(usize),
}

pub struct TCube {
    cube: Cube,
    frame: Frame,
}

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct PDR<T: SatSolver> {
    f: Vec<Vec<Cube>>,
    fin_state: FiniteStateTransitionSystem,
    solver: T,
    rng: ThreadRng,
    latch_literals: Vec<u32>,
    _input_literals: Vec<u32>,
    // caching for speedup
    initial: CNF,    // I
    transition: CNF, // T
    p0: CNF,         // P
    not_p0: CNF,     // !P
    not_p1: CNF,     // !P'
    // for printing
    verbose: bool,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl<T: SatSolver> PDR<T> {
    // ********************************************************************************************
    // helper functions - getting R_i by the definition in the paper
    // ********************************************************************************************

    // fn get_r_i(&self, k: usize) -> CNF {
    //     if k == 0 {
    //         return self.initial.to_owned();
    //     } else {
    //         let mut r_i = CNF::new();
    //         for i in k..self.f.len() {
    //             let cubes = self.f[i];
    //             for cube in &cubes {
    //                 let clause = !cube.to_owned();
    //                 r_i.add_clause(&clause);
    //             }
    //         }
    //         return r_i;
    //     }
    // }

    // // ********************************************************************************************
    // // helper functions - getting cube
    // // ********************************************************************************************

    // fn extract_cube_from_assignment(&self, assignment: &Assignment) -> Cube {
    //     let mut literals = Vec::new();

    //     for state_lit_num in &self.latch_literals {
    //         literals.push(
    //             Literal::new(state_lit_num.to_owned())
    //                 .negate_if_true(!assignment.get_value_of_variable(state_lit_num)),
    //         )
    //     }

    //     Cube::new(&literals)
    // }

    // // ********************************************************************************************
    // // helper functions - getting invariant
    // // ********************************************************************************************

    // fn get_invariant(&self) -> CNF {
    //     CNF::new()
    // }

    // // ********************************************************************************************
    // // helper functions - interface of sat queries
    // // ********************************************************************************************

    // fn get_bad_cube(&self) -> Option<Cube> {
    //     // get cube that satisfies R_N && !P
    //     let mut new_cnf = CNF::new();
    //     new_cnf.append(&self.get_r_i(self.depth()));
    //     new_cnf.append(&self.not_p0);
    //     match self.solver.solve_cnf(&new_cnf) {
    //         crate::solvers::sat::SatResponse::Sat { assignment } => {
    //             Option::Some(self.extract_cube_from_assignment(&assignment))
    //         }
    //         crate::solvers::sat::SatResponse::UnSat => Option::None,
    //     }
    // }

    // fn is_blocked(&self, s: &TCube) -> bool {
    //     match s.frame {
    //         Frame::NULL => unreachable!(),
    //         Frame::INF => unreachable!(),
    //         Frame::Ok(frame) => {
    //             let mut new_cnf = CNF::new();
    //             new_cnf.append(&self.get_r_i(frame));
    //             new_cnf.append(&s.cube.to_cnf());
    //             match self.solver.solve_cnf(&new_cnf) {
    //                 crate::solvers::sat::SatResponse::Sat { assignment } => false,
    //                 crate::solvers::sat::SatResponse::UnSat => true,
    //             }
    //         }
    //     }
    // }

    // fn is_initial(&self, c: &Cube) -> bool {
    //     // make sure all clauses in initial are in cube.
    //     for clause in self.initial.iter() {}
    // }

    // // ********************************************************************************************
    // // helper functions - functions in paper
    // // ********************************************************************************************

    // fn depth(&self) -> usize {
    //     self.f.len() - 2
    // }

    // fn new_frame(&mut self) {
    //     let n = self.f.len();
    //     self.f.push(Vec::new());
    //     self.f.swap(n - 1, n);
    // }

    // fn cond_assign(s: &mut TCube, t: TCube) -> bool {
    //     match t.frame {
    //         Frame::NULL => {
    //             // s = t;
    //             todo!();
    //             true
    //         }
    //         _ => false,
    //     }
    // }

    // // adds a cube to Fa nd the PdrSat object. It will also remove any subsumed cube in F.
    // // Subsumed cube in the Sat-Solver will be removed through periodic recycling.???????
    // fn add_blocked_cube(&self, s: &TCube) {
    //     match s.frame {
    //         Frame::Ok(s_frame_depth) => {
    //             let k = min(s_frame_depth, self.depth() + 1);

    //             // remove subsumed clauses
    //             for d in 1..(k + 1) {
    //                 let mut i = 0;
    //                 while i < self.f[d].len() {
    //                     if self.subsumes(s.cube, self.f[d][i]) {
    //                     } else {
    //                         i += 1;
    //                     }
    //                 }
    //             }
    //         }
    //         _ => {
    //             unreachable!();
    //         }
    //     }
    // }

    // fn rec_block_cube(&self, s0: TCube) -> bool {
    //     let q = BinaryHeap::<TCube>::new();
    //     q.push(s0);
    //     while q.len() > 0 {
    //         let s = q.pop().unwrap();
    //         match s.frame {
    //             Frame::Ok(s_frame) => {}
    //             _ => {
    //                 unreachable!();
    //             }
    //         }
    //     }
    //     true
    // }

    // fn propagate_blocked_cubes(&self) -> bool {
    //     // for k in 1..self.depth(){

    //     // }
    //     true
    // }

    // ********************************************************************************************
    // API functions
    // ********************************************************************************************

    pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
        let mut p0 = fin_state.get_state_to_safety_translation();
        p0.append(&fin_state.get_safety_property());

        let mut not_p0 = fin_state.get_state_to_safety_translation();
        not_p0.append(&fin_state.get_unsafety_property());

        Self {
            f: Vec::new(),
            fin_state: fin_state.to_owned(),
            solver: T::default(),
            rng: thread_rng(),
            initial: fin_state.get_initial_relation(),
            transition: fin_state.get_transition_relation(),
            p0,
            not_p0: not_p0.to_owned(),
            not_p1: fin_state.add_tags_to_relation(&not_p0, 1),
            latch_literals: fin_state.get_state_literal_numbers(),
            _input_literals: fin_state.get_input_literal_numbers(),
            verbose,
        }
    }

    pub fn pdr_main(&mut self) -> PDRResult {
        return PDRResult::CTX{ depth: 0};
        // self.f.push(Vec::new()); // push F_inf
        // self.new_frame(); // create "F[0]"

        // loop {
        //     let optional_c = self.get_bad_cube();
        //     match optional_c {
        //         Some(c) => {
        //             if !self.rec_block_cube(TCube {
        //                 cube: c,
        //                 frame: Frame::Ok(self.depth()),
        //             }) {
        //                 // failed to block 'c' => CTX found
        //                 return PDRResult::CTX {
        //                     depth: self.depth().try_into().unwrap(),
        //                 };
        //             }
        //         }
        //         None => {
        //             self.new_frame();
        //             if self.propagate_blocked_cubes() {
        //                 return PDRResult::Proof {
        //                     invariant: self.get_invariant(),
        //                 };
        //             }
        //         }
        //     }
        // }
    }
}
