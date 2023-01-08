// //! This algorithm is an exact implementation of what is described in "Efficient implementation of property directed reachability".
// //!
// //! N. Een, A. Mishchenko and R. Brayton,
// //! "Efficient implementation of property directed reachability,"
// //! 2011 Formal Methods in Computer-Aided Design (FMCAD), 2011, pp. 125-134.
// //!
// //! Abstract: Last spring, in March 2010, Aaron Bradley published the first truly new bit-level
// //! symbolic model checking algorithm since Ken McMillan's interpolation based model checking
// //! procedure introduced in 2003.
// //! Our experience with the algorithm suggests that it is stronger than interpolation on industrial
// //! problems, and that it is an important algorithm to study further.
// //! In this paper, we present a simplified and faster implementation of Bradley's procedure, and
// //! discuss our successful and unsuccessful attempts to improve it.
// //! URL: https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=6148886&isnumber=6148882
// //!
// //! The implementation of the original 2010 bit-level symbolic model checking algorithm is
// //! available under ic3 stateless solver.

// ************************************************************************************************
// use
// ************************************************************************************************

use super::{FiniteStateTransitionSystemProver, ProofResult};
use crate::{
    formulas::{literal::VariableType, Clause, Cube, Literal, CNF},
    models::FiniteStateTransitionSystem,
    solvers::sat::{
        stateful::{StatefulSatSolver, StatefulSatSolverHint},
        SatResponse,
    },
};
use priority_queue::PriorityQueue;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{cmp::{max, Reverse, min}, collections::{BinaryHeap, HashSet}};
use std::time;

// ************************************************************************************************
// Enum
// ************************************************************************************************

// enum StrengthenResult {
//     Success,
//     Failure { _depth: VariableType },
// }

// enum InductivelyGeneralizeResult {
//     Success { n: usize },
//     Failure,
// }

// enum PushGeneralizeResult {
//     Success,
//     Failure,
// }

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Frame {
    NULL,
    INF,
    Ok(usize),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct TCube {
    cube: Cube,
    frame: Frame,
}

enum SolveRelativeParam {
    DoNotExtractModel,
    ExtractModel,
    NoInd,
}



// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct PDR<T> {
    // for the algorithm
    f: Vec<Vec<Cube>>,
    fin_state: FiniteStateTransitionSystem,
    rng: ThreadRng,

    // stateful sat solvers for speedup
    // Reminder: R0 == initial
    ri_and_not_p_solvers: Vec<T>, // for each index i the solver holds Fi ^ !P
    ri_solvers: Vec<T>, // for each index i the solver holds Fi
    ri_and_t_solvers: Vec<T>, // for each index i the solver holds Fi ^ T

    // caching for speedup
    initial: CNF,
    initial_and_not_p_cnf: CNF,
    initial_and_t_cnf: CNF,
    ri_and_not_p_cnf: CNF,
    ri_and_t_cnf: CNF,
    // transition: CNF,
    // connection_from_state_to_safety0: CNF,
    // connection_from_state_to_safety1: CNF,
    // p0: Cube,
    // not_p0: Clause,
    // not_p1: Clause,
    // p_and_t: CNF,
    // p_and_t_and_not_p_tag: CNF,
    // i_and_t: CNF,
    // i_and_t_and_not_p_tag: CNF,

    // for printing
    verbose: bool,
    number_of_sat_calls: u32,
    time_in_sat_calls: time::Duration,
    start_time: time::Instant,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl<T: StatefulSatSolver> PDR<T> {

    // ********************************************************************************************
    // helper functions - TCube
    // ********************************************************************************************

    fn next(&self, s: &TCube) -> TCube {
        match s.frame {
            Frame::NULL => unreachable!(),
            Frame::INF => unreachable!(),
            Frame::Ok(i) => {
                TCube { cube: s.cube.to_owned(), frame: Frame::Ok(i + 1) }
            },
        }
    }

    // ********************************************************************************************
    // helper functions - interface of sat queries
    // ********************************************************************************************

    fn z_get_bad_cube(&mut self) -> Option<Cube> {
        // get cube that satisfies Ri && !P
        let depth = self.depth();
        match self.ri_and_not_p_solvers[depth].solve(None, None) {
            crate::solvers::sat::SatResponse::Sat { assignment } => {
                Option::Some(self.fin_state.extract_state_from_assignment(&assignment))
            }
            crate::solvers::sat::SatResponse::UnSat => Option::None,
        }
    }

    fn z_is_blocked(&mut self, s: &TCube) -> bool {
        match s.frame {
            Frame::NULL => unreachable!(),
            Frame::INF => unreachable!(),
            Frame::Ok(frame) => {
                // return true iff Ri ^ c == UnSat 
                match self.ri_solvers[frame].solve(None, None) {
                    crate::solvers::sat::SatResponse::Sat { assignment: _ } => false,
                    crate::solvers::sat::SatResponse::UnSat => true,
                }
            }
        }
    }

    fn z_is_initial(&self, c: &Cube) -> bool {
        // make sure all clauses in initial are in cube.
        self.fin_state.is_cube_initial(c)
    }

    fn z_solve_relative(&mut self, s: &TCube, params: SolveRelativeParam) -> TCube {
        match s.frame {
            Frame::NULL => unreachable!(),
            Frame::INF => unreachable!(),
            Frame::Ok(i) => {
                debug_assert!(i > 0);
                let extra_cube = self.fin_state.add_tags_to_cube(&s.cube, 1);
                let extra_clause = !s.cube.to_owned();

                let sat_result = match params {
                    SolveRelativeParam::DoNotExtractModel |
                    SolveRelativeParam::ExtractModel => {
                        // these two are the same sat call Ri-1 ^ T ^ !s.cube ^ s.cube'
                        self.ri_and_t_solvers[i-1].solve(
                            Some(&extra_cube), 
                            Some(&extra_clause)
                        )
                    }
                    SolveRelativeParam::NoInd => {
                        // this one has another sat call: Ri-1 ^ T ^ s.cube'
                        self.ri_and_t_solvers[i-1].solve(
                            Some(&extra_cube), 
                            None
                        )
                    }
                };

                match sat_result {
                    SatResponse::Sat { assignment } => {
                        match params {
                            SolveRelativeParam::DoNotExtractModel | SolveRelativeParam::NoInd => {
                                TCube { cube: Cube::new(&[]), frame: Frame::NULL }
                            },
                            SolveRelativeParam::ExtractModel => {
                                let predecessor = self.fin_state.extract_state_from_assignment(&assignment);
                                // trinary simulation todo
                                TCube { cube: predecessor, frame: Frame::NULL }
                            },
                        }
                        
                    },
                    SatResponse::UnSat => {
                        debug_assert!(s.frame != Frame::NULL);
                        s.to_owned()
                    },
                }
            },
        }
        
    }

    fn z_block_cube_in_solver(&mut self, s: &TCube) {
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_and_t_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.f.len() - 1);
        let mut cnf = CNF::new();
        cnf.add_clause(&!s.cube.to_owned());
        match s.frame {
            Frame::NULL => unreachable!(),
            Frame::INF => {
                for i in 0..self.ri_solvers.len(){
                    self.ri_solvers[i].add_cnf(&cnf);
                    self.ri_and_not_p_solvers[i].add_cnf(&cnf);
                    self.ri_and_t_solvers[i].add_cnf(&cnf);
                }
            },
            Frame::Ok(frame) => {
                debug_assert!(frame < self.ri_solvers.len());
                for i in 0..(frame + 1){
                    self.ri_solvers[i].add_cnf(&cnf);
                    self.ri_and_not_p_solvers[i].add_cnf(&cnf);
                    self.ri_and_t_solvers[i].add_cnf(&cnf);
                }
            }
        }
    }

    fn z_new_frame(&mut self) {
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_and_t_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.f.len() - 2);
        if self.ri_and_not_p_solvers.is_empty() {
            {
                let mut a = T::new(StatefulSatSolverHint::None);
                a.add_cnf(&self.initial_and_not_p_cnf);
                self.ri_and_not_p_solvers.push(a);
            } 
            {
                let mut b = T::new(StatefulSatSolverHint::None);
                b.add_cnf(&self.initial);
                self.ri_solvers.push(b);
            }
            {
                let mut c = T::new(StatefulSatSolverHint::None);
                c.add_cnf(&self.initial_and_t_cnf);
                self.ri_and_t_solvers.push(c);
            }
        } else {
            {
                let mut a = T::new(StatefulSatSolverHint::None);
                a.add_cnf(&self.ri_and_not_p_cnf);
                self.ri_and_not_p_solvers.push(a);
            } 
            {
                let b = T::new(StatefulSatSolverHint::None);
                // b.add_cnf(&self.ri);
                self.ri_solvers.push(b);
            }
            {
                let mut c = T::new(StatefulSatSolverHint::None);
                c.add_cnf(&self.ri_and_t_cnf);
                self.ri_and_t_solvers.push(c);
            }
        }
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_and_t_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.f.len() - 1);
    }

    // ********************************************************************************************
    // helper functions - helper functions in paper
    // ********************************************************************************************

    fn depth(&self) -> usize {
        self.f.len() - 2
    }

    fn new_frame(&mut self) {
        // add frame to f while moving f_inf forward.
        let n = self.f.len();
        self.f.push(Vec::new());
        self.f.swap(n - 1, n);
        self.z_new_frame();
    }

    fn is_solve_relative_un_sat(&self, t: &TCube) -> bool {
        if t.frame != Frame::NULL {
            true
        } else {
            false
        }
    }

    // adds a cube to Fa nd the PdrSat object. It will also remove any subsumed cube in F.
    // Subsumed cube in the Sat-Solver will be removed through periodic recycling.???????
    fn add_blocked_cube(&mut self, s: &TCube) {
        match s.frame {
            Frame::Ok(s_frame_depth) => {
                let k = min(s_frame_depth, self.depth() + 1);

                // remove subsumed clauses
                for d in 1..(k + 1) {
                    let mut i = 0;
                    while i < self.f[d].len() {
                        if self.subsumes(&s.cube, &self.f[d][i]) {
                            self.f[d].swap_remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }

                // store clause
                self.f[k].push(s.cube.to_owned());
                self.z_block_cube_in_solver(s)
            }
            _ => {
                unreachable!();
            }
        }
    }

    // ********************************************************************************************
    // helper functions - 2 helper functions shown in paper
    // ********************************************************************************************

    fn is_blocked(&mut self, s: &TCube) -> bool {
        match s.frame {
            Frame::Ok(s_frame) => {

                // check syntactic submission (faster than SAT)
                for d in s_frame..self.f.len(){
                    for i in 0..self.f[d].len(){
                        if self.subsumes(&self.f[d][i], &s.cube) {
                            return true;
                        }
                    }
                }

                return self.z_is_blocked(&s);
            },
            _ => unreachable!()
        }
    }

    fn generalize(&mut self, s: &TCube) -> TCube {

        let mut s_literals: Vec<Literal> = s.cube.iter().map(|l| l.to_owned()).collect();
        s_literals.shuffle(&mut self.rng);
        let mut j = 0;
        while j < s_literals.len() {
            // remove current literal
            let removed = s_literals.swap_remove(j);

            let t = TCube { cube: Cube::new(&s_literals), frame: s.frame };
            if !self.z_is_initial(&t.cube) {
                let check_if_t_is_inductive_relative_to_frame = self.z_solve_relative(&t, SolveRelativeParam::DoNotExtractModel);
                if self.is_solve_relative_un_sat(
                    &check_if_t_is_inductive_relative_to_frame
                ) {
                    // remove successful, j should remain the same
                    continue;
                }
            }

            // undo remove
            s_literals.push(removed);
            let last_index = s_literals.len() - 1;
            s_literals.swap(j, last_index);
            // move on to next literal
            j += 1;

        }
        TCube { cube: Cube::new(&s_literals), frame: s.frame }

    }

    // ********************************************************************************************
    // helper functions - last helper functions shown in paper
    // ********************************************************************************************

    fn propagate_blocked_cubes(&mut self) -> Option<usize> {
        for k in 1..self.depth() {
            let mut clause_index = 0;
            while clause_index < self.f[k].len(){
                let c = self.f[k][clause_index].to_owned();
                let s = self.z_solve_relative(
                    &TCube { cube: c, frame: Frame::Ok(k + 1) }, 
                    SolveRelativeParam::NoInd
                );
                if s.frame != Frame::NULL {
                    self.add_blocked_cube(&s);
                } else {
                    clause_index += 1;
                }
            }
            if self.f[k].is_empty(){
                return Some(k); // invariant found
            }
        }
        return None;
    }

    // ********************************************************************************************
    // helper functions - my helper functions
    // ********************************************************************************************

    fn get_r_i(&self, i: usize) -> CNF {
        assert!(self.f[0].is_empty());
        if i == 0 {
            return self.initial.to_owned();
        } else {
            let mut r_i = CNF::new();
            for i in i..self.f.len() {
                let cubes = self.f[i].to_owned();
                for cube in &cubes {
                    let clause = !cube.to_owned();
                    r_i.add_clause(&clause);
                }
            }
            return r_i;
        }
    }

    fn subsumes(&self, c1: &Cube, c2: &Cube) -> bool {
        let c1_literals = c1.iter().collect::<HashSet<&Literal>>();
        let c2_literals = c2.iter().collect::<HashSet<&Literal>>();
        return c1_literals.is_superset(&c2_literals);
    }


    // ********************************************************************************************
    // helper functions - main blocking function
    // ********************************************************************************************

    fn rec_block_cube(&mut self, s0: TCube) -> bool {
        // create queue of proof obligations.
        // Each proof obligation is a cube that reaches bad, and at what frame this cube was found.
        // It's called proof obligation because you're obliged to prove that this cube cannot be 
        // reached by previous frames.
        let mut q = PriorityQueue::<TCube, Reverse<usize>>::new();
        if let Frame::Ok(p) = s0.frame {
            q.push(s0, Reverse(p));
        } else {
            panic!("Bad Cube to block, check get_bad_cube.");
        }
        
        // while proof obligations remain.
        while q.len() > 0 {
            // take one out
            let s = q.pop().unwrap().0;
            match s.frame {
                Frame::Ok(s_frame) => {
                    if s_frame == 0 {
                        // a bad reaching cube was found in F0 == initial
                        return false;
                    } else if !self.is_blocked(&s){
                        debug_assert!(! self.z_is_initial(&s.cube));
                        let z = self.z_solve_relative(&s, SolveRelativeParam::ExtractModel);

                        if z.frame != Frame::NULL {
                            // cube 's' was blocked by image of predecessor.
                            let mut z = self.generalize(&z);
                            debug_assert!(z.frame != Frame::INF);
                            match z.frame {
                                Frame::Ok(z_frame) => {
                                    let mut another_iteration = true;
                                    while (z_frame < (self.depth() - 1)) && another_iteration {
                                        z = self.z_solve_relative(&self.next(&z), SolveRelativeParam::DoNotExtractModel);
                                        another_iteration = self.is_solve_relative_un_sat(&z);
                                    }

                                    self.add_blocked_cube(&z);

                                    if (s_frame < self.depth()) && (z.frame != Frame::INF) {
                                        let next_s = self.next(&s);
                                        match next_s.frame {
                                            Frame::Ok(next_s_frame) => {
                                                q.push(next_s, Reverse(next_s_frame));
                                            }
                                            _ => unreachable!()
                                        }
                                    }
                                },
                                _ => unreachable!(),
                            }
                            

                        } else {
                            // cube 's' was not blocked by image of predecessor.
                            match s.frame {
                                Frame::Ok(s_frame) => {
                                    debug_assert!(s_frame > 0);
                                    let z = TCube { cube: z.cube, frame: Frame::Ok(s_frame - 1) };
                                    q.push(z, Reverse(s_frame - 1));
                                    q.push(s, Reverse(s_frame));
                                },
                                _ => unreachable!(),
                            }
                        }

                    }
                }
                _ => unreachable!()
            }
        }
        true
    }

    // ********************************************************************************************
    // API functions
    // ********************************************************************************************

    pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
        let p0 = fin_state.get_safety_property();
        let not_p0 = fin_state.get_unsafety_property();
        // let not_p1 = fin_state.add_tags_to_clause(&not_p0, 1);
        let connection_from_state_to_safety0 = fin_state.get_state_to_safety_translation();
        // let connection_from_state_to_safety1 =
        //     fin_state.add_tags_to_relation(&connection_from_state_to_safety0, 1);
        let transition = fin_state.get_transition_relation();
        let initial = fin_state.get_initial_relation().to_cnf();

        let mut initial_and_not_p_cnf = CNF::new();
        initial_and_not_p_cnf.append(&initial);
        initial_and_not_p_cnf.append(&connection_from_state_to_safety0);
        initial_and_not_p_cnf.add_clause(&not_p0);

        let mut initial_and_t_cnf = CNF::new();
        initial_and_t_cnf.append(&initial);
        initial_and_t_cnf.append(&transition);

        let mut ri_and_not_p_cnf = CNF::new();
        ri_and_not_p_cnf.append(&connection_from_state_to_safety0);
        ri_and_not_p_cnf.add_clause(&not_p0);

        let mut ri_and_t_cnf = CNF::new();
        ri_and_t_cnf.append(&transition);

        Self {
            f: Vec::new(),
            fin_state: fin_state.to_owned(),
            ri_and_not_p_solvers: Vec::new(),
            ri_solvers: Vec::new(),
            ri_and_t_solvers: Vec::new(),
            // fi_and_t_and_not_p_tag_solvers: Vec::new(),
            rng: thread_rng(),
            initial,
            initial_and_not_p_cnf,
            initial_and_t_cnf,
            ri_and_not_p_cnf,
            ri_and_t_cnf,
            
            verbose,
            number_of_sat_calls: 0,
            time_in_sat_calls: time::Duration::from_secs(0),
            start_time: time::Instant::now(),
        }
    }

    pub fn prove(&mut self) -> ProofResult {
        // update start time.
        self.start_time = time::Instant::now();

        self.f.push(Vec::new()); // push F_inf
        self.new_frame(); // create "F[0]"

        loop {
            let optional_c = self.z_get_bad_cube();
            match optional_c {
                Some(c) => {
                    if !self.rec_block_cube(TCube {
                        cube: c,
                        frame: Frame::Ok(self.depth()),
                    }) {
                        // failed to block 'c' => CTX found
                        return ProofResult::CTX {
                            depth: self.depth().try_into().unwrap(),
                        };
                    }
                }
                None => {
                    self.new_frame();
                    let propagation_result = self.propagate_blocked_cubes();
                    if let Some(i) = propagation_result {
                        // invariant found may store it here.
                        return ProofResult::Proof {
                            invariant: self.get_r_i(i),
                        };
                    }
                }
            }
        }
    }
}

// ************************************************************************************************
// impl trait
// ************************************************************************************************

impl<T: StatefulSatSolver> FiniteStateTransitionSystemProver for PDR<T> {
    fn new(fin_state: &FiniteStateTransitionSystem) -> Self {
        PDR::new(fin_state, true)
    }

    fn prove(&mut self) -> ProofResult {
        self.prove()
    }
}




































// ********************************************************************************************
    // clauses
    // ********************************************************************************************

    // fn add_to_vector_of_clauses_in_specific_frame(&mut self, frame_index: usize, clause: &Clause) {
    //     debug_assert_eq!(self.clauses.len(), self.fi_and_not_p_solvers.len());
    //     debug_assert_eq!(
    //         self.clauses.len(),
    //         self.fi_and_t_and_not_p_tag_solvers.len()
    //     );

    //     self.clauses[frame_index].push(clause.to_owned());
    //     for i in 0..(frame_index + 1) {
    //         self.fi_and_not_p_solvers[i].add_cnf(&clause.to_cnf());
    //         self.fi_and_t_and_not_p_tag_solvers[i].add_cnf(&clause.to_cnf());
    //     }
    // }

    // // fn get_vector_of_clauses_in_specific_frame(&self, frame_index: usize) -> Vec<Clause> {
    // //     self.clauses[frame_index].to_owned()
    // // }

    // fn get_length_of_vector_of_clauses_in_specific_frame(&self, frame_index: usize) -> usize {
    //     self.clauses[frame_index].len()
    // }

    // fn get_clause_in_vector_of_clauses_in_specific_frame(
    //     &self,
    //     frame_index: usize,
    //     clause_index: usize,
    // ) -> Clause {
    //     self.clauses[frame_index][clause_index].to_owned()
    // }

    // fn remove_from_vector_of_clauses_in_specific_frame(
    //     &mut self,
    //     frame_index: usize,
    //     clause_index: usize,
    // ) {
    //     debug_assert_eq!(self.clauses.len(), self.fi_and_not_p_solvers.len());
    //     debug_assert_eq!(
    //         self.clauses.len(),
    //         self.fi_and_t_and_not_p_tag_solvers.len()
    //     );
    //     debug_assert!(clause_index < self.clauses[frame_index].len());

    //     self.clauses[frame_index].swap_remove(clause_index);
    // }

    // fn get_all_clause_that_are_in_some_frame(&self, frame_index: usize) -> CNF {
    //     let mut result = CNF::new();
    //     for i in frame_index..self.clauses.len() {
    //         for j in 0..self.get_length_of_vector_of_clauses_in_specific_frame(i) {
    //             let c = self.get_clause_in_vector_of_clauses_in_specific_frame(i, j);
    //             result.add_clause(&c);
    //         }
    //     }
    //     result
    // }

    // fn push_extra_frame_to_clauses(&mut self) {
    //     debug_assert_eq!(self.clauses.len(), self.fi_and_not_p_solvers.len());
    //     debug_assert_eq!(
    //         self.clauses.len(),
    //         self.fi_and_t_and_not_p_tag_solvers.len()
    //     );
    //     {
    //         // update solvers
    //         let mut a = T::new(StatefulSatSolverHint::None);
    //         a.add_cnf(if self.clauses.is_empty() {
    //             &self.i_and_t
    //         } else {
    //             &self.p_and_t
    //         });
    //         self.fi_and_not_p_solvers.push(a);
    //     }
    //     {
    //         // update solvers
    //         let mut b = T::new(StatefulSatSolverHint::None);
    //         b.add_cnf(if self.clauses.is_empty() {
    //             &self.i_and_t_and_not_p_tag
    //         } else {
    //             &self.p_and_t_and_not_p_tag
    //         });
    //         self.fi_and_t_and_not_p_tag_solvers.push(b);
    //     }

    //     self.clauses.push(Vec::new());
    // }

    // // ********************************************************************************************
    // // sat calls
    // // ********************************************************************************************

    // fn sat_call(
    //     &mut self,
    //     solver_index: SolverVariant,
    //     cube_assumptions: Option<&Cube>,
    //     clause_assumptions: Option<&Clause>,
    // ) -> SatResponse {
    //     self.number_of_sat_calls += 1;
    //     let start_time = time::Instant::now();

    //     // find solver
    //     let result = match solver_index {
    //         // SolverVariant::Initial => self
    //         //     .initial_solver
    //         //     .solve(cube_assumptions, clause_assumptions),
    //         SolverVariant::FiAndT(j) => {
    //             self.fi_and_not_p_solvers[j].solve(cube_assumptions, clause_assumptions)
    //         }
    //         SolverVariant::FiAndTAndNotPTag(j) => {
    //             self.fi_and_t_and_not_p_tag_solvers[j].solve(cube_assumptions, clause_assumptions)
    //         }
    //         SolverVariant::Custom(cnf) => {
    //             let mut current_solver = T::new(StatefulSatSolverHint::UnSat);
    //             current_solver.add_cnf(&cnf);
    //             current_solver.solve(cube_assumptions, clause_assumptions)
    //         }
    //     };

    //     self.time_in_sat_calls += start_time.elapsed();
    //     result
    // }

    // fn is_bad_reached_in_0_steps(&mut self) -> SatResponse {
    //     // I ^ !P
    //     let mut cnf = CNF::new();
    //     cnf.append(&self.initial);
    //     cnf.append(&self.connection_from_state_to_safety0);
    //     cnf.append(&self.not_p0.to_cnf());
    //     self.sat_call(SolverVariant::Custom(cnf), None, None)
    // }

    // fn is_bad_reached_in_1_steps(&mut self) -> SatResponse {
    //     // I ^ T ^ !P'
    //     let mut cnf = CNF::new();
    //     cnf.append(&self.initial);
    //     cnf.append(&self.transition);
    //     cnf.append(&self.connection_from_state_to_safety1);
    //     cnf.append(&self.not_p1.to_cnf());
    //     self.sat_call(SolverVariant::Custom(cnf), None, None)
    // }

    // fn is_cube_reachable_in_1_step_from_fi(&mut self, i: usize, cube: &Cube) -> SatResponse {
    //     // Fi ^ T ^ c'
    //     let cube_tag = self.fin_state.add_tags_to_cube(cube, 1);
    //     self.sat_call(SolverVariant::FiAndT(i), Some(&cube_tag), None)
    // }

    // fn is_bad_reachable_in_1_step_from_fi(&mut self, i: usize) -> SatResponse {
    //     // Fi ^ T ^ !P'
    //     self.sat_call(SolverVariant::FiAndTAndNotPTag(i), None, None)
    // }

    // fn is_fi_and_t_and_not_s_and_s_tag_sat(&mut self, i: usize, s: &Cube) -> bool {
    //     // Fi ^ T ^ !s ^ s'
    //     let s_tag = self.fin_state.add_tags_to_cube(s, 1);
    //     let not_s = !(s.to_owned());

    //     match self.sat_call(SolverVariant::FiAndT(i), Some(&s_tag), Some(&not_s)) {
    //         SatResponse::UnSat => false,
    //         SatResponse::Sat { assignment: _ } => true,
    //     }
    // }

    // fn is_fi_and_t_and_clause_and_not_clause_tag_sat(&mut self, i: usize, d: &Clause) -> bool {
    //     // Fi ^ T ^ d ^ !d’
    //     let not_d_tag = self.fin_state.add_tags_to_cube(&(!(d.to_owned())), 1);

    //     match self.sat_call(SolverVariant::FiAndT(i), Some(&not_d_tag), Some(d)) {
    //         SatResponse::UnSat => false,
    //         SatResponse::Sat { assignment: _ } => true,
    //     }
    // }

    // // ********************************************************************************************
    // // helper functions
    // // ********************************************************************************************

    // fn get_fk(&self, k: usize) -> CNF {
    //     let mut clauses_fk = self.get_all_clause_that_are_in_some_frame(k);
    //     if k == 0 {
    //         clauses_fk.append(&self.initial);
    //     } else {
    //         clauses_fk.append(&self.connection_from_state_to_safety0);
    //         clauses_fk.append(&self.p0.to_cnf());
    //     }
    //     clauses_fk
    // }

    // fn propagate_clauses(&mut self, k: usize) {
    //     for frame_index in 1..(k + 1) {
    //         let mut clause_index = 0;
    //         while clause_index < self.get_length_of_vector_of_clauses_in_specific_frame(frame_index)
    //         {
    //             let c = self
    //                 .get_clause_in_vector_of_clauses_in_specific_frame(frame_index, clause_index);
    //             let check =
    //                 self.is_cube_reachable_in_1_step_from_fi(frame_index, &(!(c.to_owned())));
    //             match check {
    //                 SatResponse::UnSat => {
    //                     // can propagate this property :)
    //                     self.add_to_vector_of_clauses_in_specific_frame(frame_index + 1, &c);
    //                     debug_assert_eq!(
    //                         self.get_clause_in_vector_of_clauses_in_specific_frame(
    //                             frame_index,
    //                             clause_index
    //                         )
    //                         .to_string(),
    //                         c.to_string()
    //                     );
    //                     self.remove_from_vector_of_clauses_in_specific_frame(
    //                         frame_index,
    //                         clause_index,
    //                     );
    //                 }
    //                 SatResponse::Sat { assignment: _ } => {
    //                     // can't propagate this clause :(
    //                     clause_index += 1;
    //                 }
    //             }
    //         }
    //     }
    // }

    // fn is_clause_inductive_relative_to_fi(&mut self, d: &Clause, i: usize) -> bool {
    //     // return !(Init ∧ ¬d) && !((Fi ∧ d)∧ Tr ∧ ¬d’)
    //     if self.fin_state.is_cube_initial(&(!(d.to_owned()))) {
    //         return false;
    //     }

    //     !self.is_fi_and_t_and_clause_and_not_clause_tag_sat(i, d)
    // }

    // fn get_subclause_of_not_s_that_is_inductive_relative_to_fi(
    //     &mut self,
    //     s: &Cube,
    //     i: usize,
    // ) -> Clause {
    //     let c = !(s.to_owned());
    //     let mut c_literals: Vec<Literal> = c.iter().map(|l| l.to_owned()).collect();
    //     c_literals.shuffle(&mut self.rng);
    //     let mut j = 0;
    //     while j < c_literals.len() {
    //         let removed = c_literals.swap_remove(j);
    //         let d = Clause::new(&c_literals);
    //         if self.is_clause_inductive_relative_to_fi(&d, i) {
    //             // remove successful, j should remain the same
    //         } else {
    //             // undo remove
    //             c_literals.push(removed);
    //             let last_index = c_literals.len() - 1;
    //             c_literals.swap(j, last_index);
    //             // move on to next literal
    //             j += 1;
    //         }
    //     }
    //     Clause::new(&c_literals)
    // }

    // fn generate_clause(&mut self, s: &Cube, i: usize, _k: usize) {
    //     let c = self.get_subclause_of_not_s_that_is_inductive_relative_to_fi(s, i);
    //     self.add_to_vector_of_clauses_in_specific_frame(i + 1, &c);
    // }

    // fn inductively_generalize(
    //     &mut self,
    //     s: &Cube,
    //     min: isize,
    //     k: usize,
    // ) -> Result<> {
    //     if min < 0 && self.is_fi_and_t_and_not_s_and_s_tag_sat(0, s) {
    //         return InductivelyGeneralizeResult::Failure;
    //     }

    //     for i in max(1, min + 1).try_into().unwrap()..(k + 1) {
    //         if self.is_fi_and_t_and_not_s_and_s_tag_sat(i, s) {
    //             self.generate_clause(s, i - 1, k);
    //             return InductivelyGeneralizeResult::Success { n: i - 1 };
    //         }
    //     }
    //     self.generate_clause(s, k, k);
    //     InductivelyGeneralizeResult::Success { n: k }
    // }

    // fn push_generalization(
    //     &mut self,
    //     states: &PriorityQueue<Cube, Reverse<usize>>,
    //     k: usize,
    // ) -> PushGeneralizeResult {
    //     let mut states = states.to_owned();
    //     loop {
    //         let (s, reversed_n) = states.pop().unwrap();
    //         let n = reversed_n.0;
    //         if n > k {
    //             return PushGeneralizeResult::Success;
    //         }
    //         match self.is_cube_reachable_in_1_step_from_fi(n, &s) {
    //             SatResponse::Sat { assignment } => {
    //                 // we have to block p in order to block n.
    //                 let p = self.fin_state.extract_state_from_assignment(&assignment);
    //                 // println!("Should block p = {} from F{}", p, n - 1);
    //                 match self.inductively_generalize(
    //                     &p,
    //                     <usize as TryInto<isize>>::try_into(n).unwrap() - 2,
    //                     k,
    //                 ) {
    //                     InductivelyGeneralizeResult::Failure => {
    //                         return PushGeneralizeResult::Failure;
    //                     }
    //                     InductivelyGeneralizeResult::Success { n: m } => {
    //                         states.push(s, reversed_n);
    //                         states.push(p, Reverse(m + 1));
    //                     }
    //                 }
    //             }
    //             SatResponse::UnSat => {
    //                 // n can be blocked
    //                 match self.inductively_generalize(&s, n.try_into().unwrap(), k) {
    //                     InductivelyGeneralizeResult::Failure => {
    //                         return PushGeneralizeResult::Failure;
    //                     }
    //                     InductivelyGeneralizeResult::Success { n: m } => {
    //                         states.push(s.to_owned(), Reverse(m + 1));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // fn print_progress_if_verbose(&self, k: usize) {
    //     if self.verbose {
    //         let clauses = self
    //             .clauses
    //             .iter()
    //             .map(|c| c.len())
    //             // .rev()
    //             // .take(10)
    //             .collect::<Vec<usize>>();
    //         println!("RFV1 - is on k = {}, clauses lengths = {:?}", k, clauses);
    //         println!("RFV1 - Number of SAT calls = {}", self.number_of_sat_calls);
    //         println!(
    //             "RFV1 - Time since start = {}",
    //             self.start_time.elapsed().as_secs_f32()
    //         );
    //         println!(
    //             "RFV1 - Time in SAT calls = {}",
    //             self.time_in_sat_calls.as_secs_f32()
    //         );
    //     }
    // }

    // fn strengthen(&mut self, k: usize) -> StrengthenResult {
    //     loop {
    //         match self.is_bad_reachable_in_1_step_from_fi(k) {
    //             SatResponse::UnSat => {
    //                 break;
    //             }
    //             SatResponse::Sat { assignment } => {
    //                 let s = self.fin_state.extract_state_from_assignment(&assignment);
    //                 // println!("{}", !s.to_owned());
    //                 // println!("Should block s = {} from F{}", s, k - 1);
    //                 match self.inductively_generalize(
    //                     &s,
    //                     <usize as TryInto<isize>>::try_into(k).unwrap() - 2,
    //                     k,
    //                 ) {
    //                     InductivelyGeneralizeResult::Failure => {
    //                         return StrengthenResult::Failure {
    //                             _depth: k.try_into().unwrap(),
    //                         };
    //                     }
    //                     InductivelyGeneralizeResult::Success { n } => {
    //                         let mut queue = PriorityQueue::<Cube, Reverse<usize>>::new();
    //                         queue.push(s, Reverse(n + 1));
    //                         match self.push_generalization(&queue, k) {
    //                             PushGeneralizeResult::Failure => {
    //                                 return StrengthenResult::Failure {
    //                                     _depth: k.try_into().unwrap(),
    //                                 };
    //                             }
    //                             PushGeneralizeResult::Success => {}
    //                         };
    //                     }
    //                 };
    //             }
    //         }
    //     }

    //     StrengthenResult::Success
    // }


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
// use std::{cmp::{max, Reverse}, time};
// use std::{cmp::min, collections::BinaryHeap};

// use super::ProofResult;

// // ************************************************************************************************
// // Enum
// // ************************************************************************************************

// enum Frame {
//     NULL,
//     INF,
//     Ok(usize),
// }

// struct TCube {
//     cube: Cube,
//     frame: Frame,
// }

// // ************************************************************************************************
// // struct
// // ************************************************************************************************

// pub struct PDR<T> {
//     // for the algorithm
//     clauses: Vec<Vec<Clause>>,
//     fin_state: FiniteStateTransitionSystem,
//     rng: ThreadRng,

//     // stateful sat solvers for speedup
//     // Reminder: F0 == initial
//     fi_and_not_p_solvers: Vec<T>, // for each index i the solver holds Fi ^ T
//     // initial_solver: T,        // houses just F0
//     fi_and_t_and_not_p_tag_solvers: Vec<T>, // Fi ^ T ^ !P'

//     // caching for speedup
//     initial: CNF,
//     transition: CNF,
//     connection_from_state_to_safety0: CNF,
//     connection_from_state_to_safety1: CNF,
//     p0: Cube,
//     not_p0: Clause,
//     not_p1: Clause,
//     p_and_t: CNF,
//     p_and_t_and_not_p_tag: CNF,
//     i_and_t: CNF,
//     i_and_t_and_not_p_tag: CNF,

//     // for printing
//     verbose: bool,
//     number_of_sat_calls: u32,
//     time_in_sat_calls: time::Duration,
//     start_time: time::Instant,
// }

// // ************************************************************************************************
// // impl
// // ************************************************************************************************

// impl<T: StatefulSatSolver> PDR<T> {
//     // ********************************************************************************************
//     // helper functions - getting R_i by the definition in the paper
//     // ********************************************************************************************

//     fn get_r_i(&self, k: usize) -> CNF {
//         if k == 0 {
//             return self.initial.to_owned();
//         } else {
//             let mut r_i = CNF::new();
//             for i in k..self.clauses.len() {
//                 let cubes = self.clauses[i];
//                 for cube in &cubes {
//                     let clause = !cube.to_owned();
//                     r_i.add_clause(&clause);
//                 }
//             }
//             return r_i;
//         }
//     }

//     // ********************************************************************************************
//     // helper functions - getting cube
//     // ********************************************************************************************

//     fn extract_cube_from_assignment(&self, assignment: &Assignment) -> Cube {
//         let mut literals = Vec::new();

//         for state_lit_num in &self.latch_literals {
//             literals.push(
//                 Literal::new(state_lit_num.to_owned())
//                     .negate_if_true(!assignment.get_value_of_variable(state_lit_num)),
//             )
//         }

//         Cube::new(&literals)
//     }

//     // ********************************************************************************************
//     // helper functions - getting invariant
//     // ********************************************************************************************

//     fn get_invariant(&self) -> CNF {
//         CNF::new()
//     }

//     // ********************************************************************************************
//     // helper functions - interface of sat queries
//     // ********************************************************************************************

//     fn get_bad_cube(&self) -> Option<Cube> {
//         // get cube that satisfies R_N && !P
//         let mut new_cnf = CNF::new();
//         new_cnf.append(&self.get_r_i(self.depth()));
//         new_cnf.append(&self.not_p0);
//         match self.solver.solve_cnf(&new_cnf) {
//             crate::solvers::sat::SatResponse::Sat { assignment } => {
//                 Option::Some(self.extract_cube_from_assignment(&assignment))
//             }
//             crate::solvers::sat::SatResponse::UnSat => Option::None,
//         }
//     }

//     fn is_blocked(&self, s: &TCube) -> bool {
//         match s.frame {
//             Frame::NULL => unreachable!(),
//             Frame::INF => unreachable!(),
//             Frame::Ok(frame) => {
//                 let mut new_cnf = CNF::new();
//                 new_cnf.append(&self.get_r_i(frame));
//                 new_cnf.append(&s.cube.to_cnf());
//                 match self.solver.solve_cnf(&new_cnf) {
//                     crate::solvers::sat::SatResponse::Sat { assignment } => false,
//                     crate::solvers::sat::SatResponse::UnSat => true,
//                 }
//             }
//         }
//     }

//     fn is_initial(&self, c: &Cube) -> bool {
//         // make sure all clauses in initial are in cube.
//         for clause in self.initial.iter() {}
//     }

//     // ********************************************************************************************
//     // helper functions - functions in paper
//     // ********************************************************************************************

//     fn depth(&self) -> usize {
//         self.clauses.len() - 2
//     }

//     fn new_frame(&mut self) {
//         let n = self.clauses.len();
//         self.clauses.push(Vec::new());
//         self.clauses.swap(n - 1, n);
//     }

//     fn cond_assign(s: &mut TCube, t: TCube) -> bool {
//         match t.frame {
//             Frame::NULL => {
//                 // s = t;
//                 todo!();
//                 true
//             }
//             _ => false,
//         }
//     }

//     // adds a cube to Fa nd the PdrSat object. It will also remove any subsumed cube in F.
//     // Subsumed cube in the Sat-Solver will be removed through periodic recycling.???????
//     fn add_blocked_cube(&self, s: &TCube) {
//         match s.frame {
//             Frame::Ok(s_frame_depth) => {
//                 let k = min(s_frame_depth, self.depth() + 1);

//                 // remove subsumed clauses
//                 for d in 1..(k + 1) {
//                     let mut i = 0;
//                     while i < self.clauses[d].len() {
//                         if self.subsumes(s.cube, self.clauses[d][i]) {
//                         } else {
//                             i += 1;
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 unreachable!();
//             }
//         }
//     }

//     fn rec_block_cube(&self, s0: TCube) -> bool {
//         let q = BinaryHeap::<TCube>::new();
//         q.push(s0);
//         while q.len() > 0 {
//             let s = q.pop().unwrap();
//             match s.frame {
//                 Frame::Ok(s_frame) => {}
//                 _ => {
//                     unreachable!();
//                 }
//             }
//         }
//         true
//     }

//     fn propagate_blocked_cubes(&self) -> bool {
//         // for k in 1..self.depth(){

//         // }
//         true
//     }

//     // ********************************************************************************************
//     // API functions
//     // ********************************************************************************************

//     pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
//         let p0 = fin_state.get_safety_property();
//         let not_p0 = fin_state.get_unsafety_property();
//         let not_p1 = fin_state.add_tags_to_clause(&not_p0, 1);
//         let connection_from_state_to_safety0 = fin_state.get_state_to_safety_translation();
//         let connection_from_state_to_safety1 =
//             fin_state.add_tags_to_relation(&connection_from_state_to_safety0, 1);
//         let transition = fin_state.get_transition_relation();
//         let initial = fin_state.get_initial_relation().to_cnf();

//         let mut i_and_t = CNF::new();
//         i_and_t.append(&initial);
//         i_and_t.append(&transition);

//         let mut i_and_t_and_not_p_tag = CNF::new();
//         i_and_t_and_not_p_tag.append(&i_and_t);
//         i_and_t_and_not_p_tag.append(&connection_from_state_to_safety1);
//         i_and_t_and_not_p_tag.append(&not_p1.to_cnf());

//         let mut p_and_t = CNF::new();
//         p_and_t.append(&connection_from_state_to_safety0);
//         p_and_t.append(&p0.to_cnf());
//         p_and_t.append(&transition);

//         let mut p_and_t_and_not_p_tag = CNF::new();
//         p_and_t_and_not_p_tag.append(&p_and_t);
//         p_and_t_and_not_p_tag.append(&connection_from_state_to_safety1);
//         p_and_t_and_not_p_tag.append(&not_p1.to_cnf());

//         Self {
//             clauses: Vec::new(),
//             fin_state: fin_state.to_owned(),
//             fi_and_not_p_solvers: Vec::new(),
//             fi_and_t_and_not_p_tag_solvers: Vec::new(),
//             rng: thread_rng(),
//             initial,
//             transition,
//             p0,
//             not_p0,
//             not_p1,
//             connection_from_state_to_safety0,
//             connection_from_state_to_safety1,
//             p_and_t,
//             p_and_t_and_not_p_tag,
//             i_and_t,
//             i_and_t_and_not_p_tag,
//             verbose,
//             number_of_sat_calls: 0,
//             time_in_sat_calls: time::Duration::from_secs(0),
//             start_time: time::Instant::now(),
//         }
//     }

//     pub fn pdr_main(&mut self) -> ProofResult {
//         return ProofResult::CTX{ depth: 0};
//         // self.f.push(Vec::new()); // push F_inf
//         // self.new_frame(); // create "F[0]"

//         // loop {
//         //     let optional_c = self.get_bad_cube();
//         //     match optional_c {
//         //         Some(c) => {
//         //             if !self.rec_block_cube(TCube {
//         //                 cube: c,
//         //                 frame: Frame::Ok(self.depth()),
//         //             }) {
//         //                 // failed to block 'c' => CTX found
//         //                 return PDRResult::CTX {
//         //                     depth: self.depth().try_into().unwrap(),
//         //                 };
//         //             }
//         //         }
//         //         None => {
//         //             self.new_frame();
//         //             if self.propagate_blocked_cubes() {
//         //                 return PDRResult::Proof {
//         //                     invariant: self.get_invariant(),
//         //                 };
//         //             }
//         //         }
//         //     }
//         // }
//     }
// }
