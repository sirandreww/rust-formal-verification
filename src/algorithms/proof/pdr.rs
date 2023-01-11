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
    formulas::{Cube, Literal, CNF},
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
use std::{
    cmp::{min, Reverse},
    collections::HashSet,
};
use std::{collections::HashMap, time};

// ************************************************************************************************
// Enum
// ************************************************************************************************

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Frame {
    Null,
    Inf,
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
    ri_solvers: Vec<T>,           // for each index i the solver holds Fi
    ri_and_t_solvers: Vec<T>,     // for each index i the solver holds Fi ^ T

    // caching for speedup
    initial: CNF,
    initial_and_not_p_cnf: CNF,
    initial_and_t_cnf: CNF,
    ri_and_not_p_cnf: CNF,
    ri_and_t_cnf: CNF,

    // checks that skip useless cube intersection
    should_intersect_predecessor_with_transition_cone: bool,
    should_intersect_cube_with_safety_cone: bool,

    // for printing
    verbose: bool,
    sat_call_stats: HashMap<String, (u32, f32)>,

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
            Frame::Null => unreachable!(),
            Frame::Inf => unreachable!(),
            Frame::Ok(i) => TCube {
                cube: s.cube.to_owned(),
                frame: Frame::Ok(i + 1),
            },
        }
    }

    // ********************************************************************************************
    // helper functions - interface of sat queries
    // ********************************************************************************************

    fn z_get_bad_cube(&mut self) -> Option<Cube> {
        // get cube that satisfies Ri && !P
        let depth = self.depth();

        self.sat_call_stats.get_mut("z_get_bad_cube").unwrap().0 += 1;
        let start_time = time::Instant::now();
        let sat_response = self.ri_and_not_p_solvers[depth].solve(None, None);
        self.sat_call_stats.get_mut("z_get_bad_cube").unwrap().1 +=
            start_time.elapsed().as_secs_f32();

        match sat_response {
            SatResponse::Sat { assignment } => {
                let mut bad_state = self.fin_state.extract_state_from_assignment(&assignment);

                if self.should_intersect_cube_with_safety_cone {
                    let size_before = bad_state.len();
                    bad_state = self
                        .fin_state
                        .intersect_cube_with_cone_of_safety(&bad_state);
                    let size_after = bad_state.len();
                    debug_assert!(size_after <= size_before);
                    self.should_intersect_cube_with_safety_cone = size_before != size_after;
                }

                Option::Some(bad_state)
            }
            SatResponse::UnSat => Option::None,
        }
    }

    fn z_is_blocked(&mut self, s: &TCube) -> bool {
        match s.frame {
            Frame::Null => unreachable!(),
            Frame::Inf => unreachable!(),
            Frame::Ok(frame) => {
                // return true iff Ri ^ c == UnSat
                self.sat_call_stats.get_mut("z_is_blocked").unwrap().0 += 1;
                let start_time = time::Instant::now();
                let sat_response = self.ri_solvers[frame].solve(None, None);
                self.sat_call_stats.get_mut("z_is_blocked").unwrap().1 +=
                    start_time.elapsed().as_secs_f32();

                match sat_response {
                    SatResponse::Sat { assignment: _ } => false,
                    SatResponse::UnSat => true,
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
            Frame::Null => unreachable!(),
            Frame::Inf => unreachable!(),
            Frame::Ok(i) => {
                debug_assert!(0 < i && i < self.ri_and_t_solvers.len());
                let extra_cube = self.fin_state.add_tags_to_cube(&s.cube, 1);
                let extra_clause = !s.cube.to_owned();

                // update number of calls for sat solver stats
                match params {
                    SolveRelativeParam::DoNotExtractModel => {
                        self.sat_call_stats
                            .get_mut("z_solve_relative_DoNotExtractModel")
                            .unwrap()
                            .0 += 1
                    }
                    SolveRelativeParam::NoInd => {
                        self.sat_call_stats
                            .get_mut("z_solve_relative_NoInd")
                            .unwrap()
                            .0 += 1
                    }
                    SolveRelativeParam::ExtractModel => {
                        self.sat_call_stats
                            .get_mut("z_solve_relative_ExtractModel")
                            .unwrap()
                            .0 += 1
                    }
                }

                let start_time = time::Instant::now();
                let sat_result = match params {
                    SolveRelativeParam::DoNotExtractModel | SolveRelativeParam::ExtractModel => {
                        // these two are the same sat call Ri-1 ^ T ^ !s.cube ^ s.cube'
                        self.ri_and_t_solvers[i - 1].solve(Some(&extra_cube), Some(&extra_clause))
                    }
                    SolveRelativeParam::NoInd => {
                        // this one has another sat call: Ri-1 ^ T ^ s.cube'
                        self.ri_and_t_solvers[i - 1].solve(Some(&extra_cube), None)
                    }
                };

                // update time of calls for sat solver stats
                match params {
                    SolveRelativeParam::DoNotExtractModel => {
                        self.sat_call_stats
                            .get_mut("z_solve_relative_DoNotExtractModel")
                            .unwrap()
                            .1 += start_time.elapsed().as_secs_f32()
                    }
                    SolveRelativeParam::NoInd => {
                        self.sat_call_stats
                            .get_mut("z_solve_relative_NoInd")
                            .unwrap()
                            .1 += start_time.elapsed().as_secs_f32()
                    }
                    SolveRelativeParam::ExtractModel => {
                        self.sat_call_stats
                            .get_mut("z_solve_relative_ExtractModel")
                            .unwrap()
                            .1 += start_time.elapsed().as_secs_f32()
                    }
                }

                match sat_result {
                    SatResponse::Sat { assignment } => {
                        match params {
                            SolveRelativeParam::DoNotExtractModel | SolveRelativeParam::NoInd => {
                                TCube {
                                    cube: Cube::new(&[]),
                                    frame: Frame::Null,
                                }
                            }
                            SolveRelativeParam::ExtractModel => {
                                let mut predecessor =
                                    self.fin_state.extract_state_from_assignment(&assignment);

                                if self.should_intersect_predecessor_with_transition_cone {
                                    let size_before = predecessor.len();

                                    predecessor = self
                                        .fin_state
                                        .intersect_cube_with_cone_of_transition(&predecessor);

                                    let size_after = predecessor.len();
                                    debug_assert!(size_after <= size_before);

                                    self.should_intersect_predecessor_with_transition_cone =
                                        size_before != size_after;
                                }

                                // trinary simulation todo
                                TCube {
                                    cube: predecessor,
                                    frame: Frame::Null,
                                }
                            }
                        }
                    }
                    SatResponse::UnSat => {
                        debug_assert!(s.frame != Frame::Null);
                        s.to_owned()
                    }
                }
            }
        }
    }

    fn z_block_cube_in_solver(&mut self, s: &TCube) {
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.ri_and_t_solvers.len());
        debug_assert_eq!(self.ri_and_not_p_solvers.len(), self.f.len() - 1);
        let mut cnf = CNF::new();
        cnf.add_clause(&!s.cube.to_owned());
        match s.frame {
            Frame::Null => unreachable!(),
            Frame::Inf => {
                for i in 0..self.ri_solvers.len() {
                    self.ri_solvers[i].add_cnf(&cnf);
                    self.ri_and_not_p_solvers[i].add_cnf(&cnf);
                    self.ri_and_t_solvers[i].add_cnf(&cnf);
                }
            }
            Frame::Ok(frame) => {
                debug_assert!(frame < self.ri_solvers.len());
                for i in 0..(frame + 1) {
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
        t.frame != Frame::Null
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
                for d in s_frame..self.f.len() {
                    for i in 0..self.f[d].len() {
                        if self.subsumes(&self.f[d][i], &s.cube) {
                            return true;
                        }
                    }
                }

                self.z_is_blocked(s)
            }
            _ => unreachable!(),
        }
    }

    fn generalize(&mut self, s: &TCube) -> TCube {
        let mut s_literals: Vec<Literal> = s.cube.iter().map(|l| l.to_owned()).collect();
        s_literals.shuffle(&mut self.rng);
        let mut j = 0;
        while j < s_literals.len() {
            // remove current literal
            let removed = s_literals.swap_remove(j);

            let t = TCube {
                cube: Cube::new(&s_literals),
                frame: s.frame,
            };
            if !self.z_is_initial(&t.cube) {
                let check_if_t_is_inductive_relative_to_frame =
                    self.z_solve_relative(&t, SolveRelativeParam::DoNotExtractModel);
                if self.is_solve_relative_un_sat(&check_if_t_is_inductive_relative_to_frame) {
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
        TCube {
            cube: Cube::new(&s_literals),
            frame: s.frame,
        }
    }

    // ********************************************************************************************
    // helper functions - last helper functions shown in paper
    // ********************************************************************************************

    fn propagate_blocked_cubes(&mut self) -> Option<usize> {
        for k in 1..self.depth() {
            let mut clause_index = 0;
            while clause_index < self.f[k].len() {
                let c = self.f[k][clause_index].to_owned();
                let s = self.z_solve_relative(
                    &TCube {
                        cube: c,
                        frame: Frame::Ok(k + 1),
                    },
                    SolveRelativeParam::NoInd,
                );
                if s.frame != Frame::Null {
                    self.add_blocked_cube(&s);
                } else {
                    clause_index += 1;
                }
            }
            if self.f[k].is_empty() {
                return Some(k); // invariant found
            }
        }
        None
    }

    // ********************************************************************************************
    // helper functions - my helper functions
    // ********************************************************************************************

    fn get_r_i(&self, i: usize) -> CNF {
        println!("get_r_i called with i = {}", i);
        assert!(self.f[0].is_empty());
        let mut r_i = CNF::new();
        if i == 0 {
            r_i.append(&self.initial);
        } else {
            for i in i..self.f.len() {
                let cubes = self.f[i].to_owned();
                for cube in &cubes {
                    let clause = !cube.to_owned();
                    r_i.add_clause(&clause);
                }
            }
        }
        r_i
    }

    fn subsumes(&self, c1: &Cube, c2: &Cube) -> bool {
        let c1_literals = c1.iter().collect::<HashSet<&Literal>>();
        let c2_literals = c2.iter().collect::<HashSet<&Literal>>();
        c1_literals.is_subset(&c2_literals)
    }

    fn print_progress_if_verbose(&self) {
        if self.verbose {
            let clauses = self
                .f
                .iter()
                .map(|c| c.len())
                // .rev()
                // .take(10)
                .collect::<Vec<usize>>();
            println!(
                "PDR - is on depth = {}, clauses lengths = {:?}",
                self.depth(),
                clauses
            );
            println!(
                "PDR - Time since start = {}",
                self.start_time.elapsed().as_secs_f32()
            );
            println!("PDR - Sat call stats:");
            for (key, value) in self.sat_call_stats.iter() {
                println!("PDR - {}: {:?}", key, value);
            }
        }
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
        while !q.is_empty() {
            // take one out
            let s = q.pop().unwrap().0;
            match s.frame {
                Frame::Ok(s_frame) => {
                    if s_frame == 0 {
                        // a bad reaching cube was found in F0 == initial
                        return false;
                    } else if !self.is_blocked(&s) {
                        debug_assert!(!self.z_is_initial(&s.cube));
                        let z = self.z_solve_relative(&s, SolveRelativeParam::ExtractModel);

                        if z.frame != Frame::Null {
                            // cube 's' was blocked by image of predecessor.
                            let mut z = self.generalize(&z);
                            debug_assert!(z.frame != Frame::Inf);
                            match z.frame {
                                Frame::Ok(mut z_frame) => {
                                    let mut another_iteration = true;
                                    while (z_frame < (self.depth() - 1)) && another_iteration {
                                        let potential_z = self.z_solve_relative(
                                            &self.next(&z),
                                            SolveRelativeParam::DoNotExtractModel,
                                        );
                                        another_iteration =
                                            self.is_solve_relative_un_sat(&potential_z);
                                        if another_iteration {
                                            z = potential_z;
                                            debug_assert!(z.frame != Frame::Inf);
                                            debug_assert!(z.frame != Frame::Null);
                                            match z.frame {
                                                Frame::Ok(potential_z_frame) => {
                                                    z_frame = potential_z_frame
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                    debug_assert!(z.frame != Frame::Inf);
                                    debug_assert!(z.frame != Frame::Null);
                                    self.add_blocked_cube(&z);

                                    debug_assert!(z.frame != Frame::Inf);
                                    if (s_frame < self.depth()) && (z.frame != Frame::Inf) {
                                        let next_s = self.next(&s);
                                        match next_s.frame {
                                            Frame::Ok(next_s_frame) => {
                                                q.push(next_s, Reverse(next_s_frame));
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            // cube 's' was not blocked by image of predecessor.
                            match s.frame {
                                Frame::Ok(s_frame) => {
                                    debug_assert!(s_frame > 0);
                                    let z = TCube {
                                        cube: z.cube,
                                        frame: Frame::Ok(s_frame - 1),
                                    };
                                    q.push(z, Reverse(s_frame - 1));
                                    q.push(s, Reverse(s_frame));
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        true
    }

    // ********************************************************************************************
    // API functions
    // ********************************************************************************************

    pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
        // let p0 = fin_state.get_safety_property();
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

        let sat_call_stats = HashMap::from([
            ("z_get_bad_cube".to_string(), (0, 0_f32)),
            ("z_is_blocked".to_string(), (0, 0_f32)),
            ("z_solve_relative_DoNotExtractModel".to_string(), (0, 0_f32)),
            ("z_solve_relative_NoInd".to_string(), (0, 0_f32)),
            ("z_solve_relative_ExtractModel".to_string(), (0, 0_f32)),
        ]);

        Self {
            f: Vec::new(),
            fin_state: fin_state.to_owned(),
            ri_and_not_p_solvers: Vec::new(),
            ri_solvers: Vec::new(),
            ri_and_t_solvers: Vec::new(),
            rng: thread_rng(),
            initial,
            initial_and_not_p_cnf,
            initial_and_t_cnf,
            ri_and_not_p_cnf,
            ri_and_t_cnf,
            should_intersect_predecessor_with_transition_cone: true,
            should_intersect_cube_with_safety_cone: true,
            verbose,
            start_time: time::Instant::now(),
            sat_call_stats,
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
                    self.print_progress_if_verbose();
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
