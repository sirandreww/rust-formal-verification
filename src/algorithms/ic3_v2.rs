//! This algorithm is an almost exact implementation of what is described in "SAT-Based Model Checking without Unrolling".
//!
//! Bradley, A.R. (2011). SAT-Based Model Checking without Unrolling.
//! In: Jhala, R., Schmidt, D. (eds) Verification, Model Checking, and Abstract Interpretation.
//! VMCAI 2011. Lecture Notes in Computer Science, vol 6538. Springer, Berlin,
//! Heidelberg. https://doi.org/10.1007/978-3-642-18275-4_7
//!
//! Abstract: A new form of SAT-based symbolic model checking is described.
//! Instead of unrolling the transition relation, it incrementally generates clauses that are
//! inductive relative to (and augment) stepwise approximate reachability information.
//! In this way, the algorithm gradually refines the property, eventually producing either an
//! inductive strengthening of the property or a counterexample trace. Our experimental studies
//! show that induction is a powerful tool for generalizing the unreachability of given error
//! states: it can refine away many states at once, and it is effective at focusing the proof
//! search on aspects of the transition system relevant to the property. Furthermore, the
//! incremental structure of the algorithm lends itself to a parallel implementation.
//!
//! This implementation uses stateful sat solvers to gain speedup.

// ************************************************************************************************
// use
// ************************************************************************************************

use crate::{
    formulas::{literal::VariableType, Clause, Cube, Literal, CNF},
    models::FiniteStateTransitionSystem,
    solvers::sat::{stateful::StatefulSatSolver, Assignment, SatResponse, StatelessSatSolver},
};
use priority_queue::PriorityQueue;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{max, Reverse};
use std::time;

// ************************************************************************************************
// Enum
// ************************************************************************************************

pub enum IC3V2Result {
    Proof { invariant: CNF },
    CTX { depth: VariableType },
}

enum StrengthenResult {
    Success,
    Failure { depth: VariableType },
}

enum InductivelyGeneralizeResult {
    Success { n: usize },
    Failure,
}

enum PushGeneralizeResult {
    Success,
    Failure,
}

enum SolverVariant {
    Initial,
    FiAndT(usize),
    FiAndTAndNotPTag(usize),
}

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct IC3V2<T> {
    // for the algorithm
    clauses: Vec<CNF>,
    fin_state: FiniteStateTransitionSystem,
    rng: ThreadRng,
    latch_literals: Vec<u32>,
    _input_literals: Vec<u32>,

    // stateful sat solvers for speedup
    // Reminder: F0 == initial
    fi_and_t_solvers: Vec<T>, // for each index i the solver holds Fi ^ T
    initial_solver: T,        // houses just F0
    fi_and_t_and_not_p_tag_solvers: Vec<T>, // Fi ^ T ^ !P'

    // caching for speedup
    initial: CNF,
    transition: CNF,
    p0: CNF,
    not_p0: CNF,
    not_p1: CNF,

    // for printing
    verbose: bool,
    number_of_sat_calls: u32,
    time_in_sat_calls: time::Duration,
    start_time: time::Instant,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl<T: StatefulSatSolver> IC3V2<T> {
    // ********************************************************************************************
    // clauses
    // ********************************************************************************************

    fn add_clause_to_clauses(&mut self, index: usize, clause: &Clause) {
        self.clauses[index].add_clause(clause);
    }

    fn get_clause_from_clauses(&self, index: usize) -> CNF {
        self.clauses[index]
    }

    fn push_extra_frame_to_clauses(&mut self) {
        self.clauses.push(CNF::new());
    }

    // ********************************************************************************************
    // sat calls
    // ********************************************************************************************

    fn sat_call(
        &mut self,
        solver_index: SolverVariant,
        assumptions: Option<&Cube>,
        temporary_cnf: Option<&CNF>, // costly so please avoid.
    ) -> SatResponse {
        self.number_of_sat_calls += 1;
        let start_time = time::Instant::now();

        // find solver
        let mut solver = match solver_index {
            SolverVariant::Initial => &self.initial_solver,
            SolverVariant::FiAndT(j) => &self.fi_and_t_solvers[j],
            SolverVariant::FiAndTAndNotPTag(j) => &self.fi_and_t_and_not_p_tag_solvers[j],
        };

        // clone and add temporary cnf if needed.
        match temporary_cnf {
            Some(cnf) => {
                solver = &solver.clone();
                solver.add_cnf(cnf);
            }
            None => {}
        };

        // call it
        let result = match assumptions {
            Some(c_assumption) => solver.solve_under_assumptions(c_assumption),
            None => solver.solve(),
        };

        self.time_in_sat_calls += start_time.elapsed();
        result
    }

    fn is_bad_reached_in_0_steps(&mut self) -> SatResponse {
        // I ^ !P
        self.sat_call(SolverVariant::Initial, None, Some(&self.not_p0))
    }

    fn is_bad_reached_in_1_steps(&mut self) -> SatResponse {
        // I ^ T ^ !P'
        self.sat_call(SolverVariant::FiAndT(0), None, Some(&self.not_p1))
    }

    fn is_cube_reachable_in_1_step_from_fi(&mut self, i: usize, cube: &Cube) -> SatResponse {
        // Fi ^ T ^ c'
        let cube_tag = self.fin_state.add_tags_to_cube(cube, 1);
        self.sat_call(SolverVariant::FiAndT(i), Some(&cube_tag), None)
    }

    fn is_bad_reachable_in_1_step_from_fi(&mut self, i: usize) -> SatResponse {
        // Fi ^ T ^ !P'
        self.sat_call(SolverVariant::FiAndTAndNotPTag(i), None, None)
    }

    fn is_fi_and_t_and_not_s_and_s_tag_sat(&mut self, i: usize, s: &Cube) -> bool {
        // Fi ^ T ^ !s ^ s')
        // this implementation can be improved since now stateful solver is copied each time.
        let s_tag = self.fin_state.add_tags_to_cube(s, 1);
        let not_s = !(s.to_owned());

        match self.sat_call(SolverVariant::FiAndTAndNotPTag(i), Some(&s_tag), Some()) {
            SatResponse::UnSat => false,
            SatResponse::Sat { assignment: _ } => true,
        }
    }

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn get_fk(&self, k: usize) -> CNF {
        let mut clauses_fk = self.get_clause_from_clauses(k);
        if k == 0 {
            clauses_fk.append(&self.initial);
        } else {
            clauses_fk.append(&self.p0);
        }
        clauses_fk
    }

    fn propagate_clauses(&mut self, k: usize) {
        for i in 1..(k + 1) {
            let clauses_fi = self.get_clause_from_clauses(i);
            for c in clauses_fi.iter() {
                let check = self.is_cube_reachable_in_1_step_from_fi(i, &(!(c.to_owned())));
                match check {
                    SatResponse::UnSat => {
                        // can propagate this property :)
                        self.add_clause_to_clauses(i + 1, c);
                    }
                    SatResponse::Sat { assignment: _ } => {
                        // can't propagate this clause :(
                    }
                }
            }
        }
    }

    fn extract_predecessor_from_assignment(&self, assignment: &Assignment) -> Cube {
        let mut literals = Vec::new();

        for state_lit_num in &self.latch_literals {
            literals.push(
                Literal::new(state_lit_num.to_owned())
                    .negate_if_true(!assignment.get_value_of_variable(state_lit_num)),
            )
        }

        Cube::new(&literals)
    }

    fn is_clause_inductive_relative_to_fi(&mut self, d: &Clause, i: usize) -> bool {
        // return !(Init ∧ ¬d) && !((Fi ∧ d)∧ Tr ∧ ¬d’)
        if self.fin_state.is_cube_initial(&(!(d.to_owned()))) {
            let mut first_cnf = self.initial.to_owned();
            first_cnf.append(&(!d.to_owned()).to_cnf());
            match self.sat_call(&first_cnf) {
                SatResponse::UnSat => {}
                SatResponse::Sat { assignment: _ } => {
                    return false;
                }
            }
            return false;
        }

        let mut second_cnf = self.get_fk(i);
        second_cnf.add_clause(d);
        second_cnf.append(&self.transition);
        second_cnf.append(
            &self
                .fin_state
                .add_tags_to_relation(&(!d.to_owned()).to_cnf(), 1),
        );

        let not_d_tag = self.fin_state.add_tags_to_cube(&(!(d.to_owned())), 1);

        match self.sat_call(&not_d_tag) {
            SatResponse::UnSat => true,
            SatResponse::Sat { assignment: _ } => false,
        }
    }

    fn get_subclause_of_not_s_that_is_inductive_relative_to_fi(
        &mut self,
        s: &Cube,
        i: usize,
    ) -> Clause {
        let c = !(s.to_owned());
        let mut c_literals: Vec<Literal> = c.iter().map(|l| l.to_owned()).collect();
        c_literals.shuffle(&mut self.rng);
        let mut j = 0;
        while j < c_literals.len() {
            let removed = c_literals.swap_remove(j);
            let d = Clause::new(&c_literals);
            if self.is_clause_inductive_relative_to_fi(&d, i) {
                // remove successful, j should remain the same
            } else {
                // undo remove
                c_literals.push(removed);
                let last_index = c_literals.len() - 1;
                c_literals.swap(j, last_index);
                // move on to next literal
                j += 1;
            }
        }
        Clause::new(&c_literals)
    }

    fn generate_clause(&mut self, s: &Cube, i: usize, _k: usize) {
        let c = self.get_subclause_of_not_s_that_is_inductive_relative_to_fi(s, i);
        for j in 1..(i + 2) {
            self.add_clause_to_clauses(j, &c);
        }
    }

    fn inductively_generalize(
        &mut self,
        s: &Cube,
        min: isize,
        k: usize,
    ) -> InductivelyGeneralizeResult {
        if min < 0 && self.is_fi_and_t_and_not_s_and_s_tag_sat(0, s) {
            return InductivelyGeneralizeResult::Failure;
        }

        for i in max(1, min + 1).try_into().unwrap()..(k + 1) {
            if self.is_fi_and_t_and_not_s_and_s_tag_sat(i, s) {
                self.generate_clause(s, i - 1, k);
                return InductivelyGeneralizeResult::Success { n: i - 1 };
            }
        }
        self.generate_clause(s, k, k);
        InductivelyGeneralizeResult::Success { n: k }
    }

    // calculates sat(Fi ^ T ^ s')
    fn solve_fi_and_t_and_s_tag(&mut self, i: usize, s: &Cube) -> SatResponse {
        let assumptions = self.fin_state.add_tags_to_cube(s, 1);
        self.sat_call(&assumptions, SolverVariant::FiAndT(i))
    }

    fn push_generalization(
        &mut self,
        states: &PriorityQueue<Cube, Reverse<usize>>,
        k: usize,
    ) -> PushGeneralizeResult {
        let mut states = states.to_owned();
        loop {
            let (s, reversed_n) = states.pop().unwrap();
            let n = reversed_n.0;
            if n > k {
                return PushGeneralizeResult::Success;
            }
            match self.solve_fi_and_t_and_s_tag(n, &s) {
                SatResponse::Sat { assignment } => {
                    // we have to block p in order to block n.
                    let p = self.extract_predecessor_from_assignment(&assignment);
                    // println!("Should block p = {} from F{}", p, n - 1);
                    match self.inductively_generalize(
                        &p,
                        <usize as TryInto<isize>>::try_into(n).unwrap() - 2,
                        k,
                    ) {
                        InductivelyGeneralizeResult::Failure => {
                            return PushGeneralizeResult::Failure;
                        }
                        InductivelyGeneralizeResult::Success { n: m } => {
                            states.push(s, reversed_n);
                            states.push(p, Reverse(m + 1));
                        }
                    }
                }
                SatResponse::UnSat => {
                    // n can be blocked
                    match self.inductively_generalize(&s, n.try_into().unwrap(), k) {
                        InductivelyGeneralizeResult::Failure => {
                            return PushGeneralizeResult::Failure;
                        }
                        InductivelyGeneralizeResult::Success { n: m } => {
                            states.push(s.to_owned(), Reverse(m + 1));
                        }
                    }
                }
            }
        }
    }

    fn print_progress_if_verbose(&self, k: usize) {
        if self.verbose {
            let clauses = self
                .clauses
                .iter()
                .map(|c| c.len())
                .rev()
                .take(10)
                .collect::<Vec<usize>>();
            println!("IC3 is on k = {}, clauses lengths = {:?}", k, clauses);
            println!("Number of SAT calls = {}", self.number_of_sat_calls);
            println!(
                "Time since start = {}",
                self.start_time.elapsed().as_secs_f32()
            );
            println!(
                "Time in SAT calls = {}",
                self.time_in_sat_calls.as_secs_f32()
            );
        }
    }

    fn strengthen(&mut self, k: usize) -> StrengthenResult {
        loop {
            match self.is_bad_reachable_in_1_step_from_fi(k) {
                SatResponse::UnSat => {
                    break;
                }
                SatResponse::Sat { assignment } => {
                    let s = self.extract_predecessor_from_assignment(&assignment);
                    // println!("Should block s = {} from F{}", s, k - 1);
                    match self.inductively_generalize(
                        &s,
                        <usize as TryInto<isize>>::try_into(k).unwrap() - 2,
                        k,
                    ) {
                        InductivelyGeneralizeResult::Failure => {
                            return StrengthenResult::Failure {
                                depth: k.try_into().unwrap(),
                            };
                        }
                        InductivelyGeneralizeResult::Success { n } => {
                            let mut queue = PriorityQueue::<Cube, Reverse<usize>>::new();
                            queue.push(s, Reverse(n + 1));
                            match self.push_generalization(&queue, k) {
                                PushGeneralizeResult::Failure => {
                                    return StrengthenResult::Failure {
                                        depth: k.try_into().unwrap(),
                                    };
                                }
                                PushGeneralizeResult::Success => {}
                            };
                        }
                    };
                }
            }
        }

        StrengthenResult::Success
    }

    // ********************************************************************************************
    // API functions
    // ********************************************************************************************

    pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
        let mut p0 = fin_state.get_state_to_safety_translation();
        p0.append(&fin_state.get_safety_property());

        let mut not_p0 = fin_state.get_state_to_safety_translation();
        not_p0.append(&fin_state.get_unsafety_property());

        Self {
            clauses: Vec::new(),
            fin_state: fin_state.to_owned(),
            fi_and_t_solvers: Vec::new(),
            initial_solver: T::default(),
            rng: thread_rng(),
            initial: fin_state.get_initial_relation().to_cnf(),
            transition: fin_state.get_transition_relation(),
            p0,
            not_p0: not_p0.to_owned(),
            not_p1: fin_state.add_tags_to_relation(&not_p0, 1),
            latch_literals: fin_state.get_state_literal_numbers(),
            _input_literals: fin_state.get_input_literal_numbers(),
            verbose,
            number_of_sat_calls: 0,
            time_in_sat_calls: time::Duration::from_secs(0),
            start_time: time::Instant::now(),
        }
    }

    pub fn prove(&mut self) -> IC3V2Result {
        // update start time.
        self.start_time = time::Instant::now();

        let init_and_not_p = self.is_bad_reached_in_0_steps();
        match init_and_not_p {
            SatResponse::Sat { assignment: _ } => return IC3V2Result::CTX { depth: 0 },
            SatResponse::UnSat => (),
        }
        // debug_assert!(does_a_imply_b::<T>(&self.initial, &self.p0));

        let init_and_tr_and_not_p_tag = self.is_bad_reached_in_1_steps();
        match init_and_tr_and_not_p_tag {
            SatResponse::Sat { assignment: _ } => return IC3V2Result::CTX { depth: 1 },
            SatResponse::UnSat => (),
        }

        self.push_extra_frame_to_clauses();
        self.push_extra_frame_to_clauses();
        for k in 1.. {
            self.push_extra_frame_to_clauses();
            // debug_assert!(self.does_a_hold(k), "Bug in algorithm implementation found!!");
            self.print_progress_if_verbose(k);
            debug_assert_eq!(self.clauses.len(), (k + 2));
            debug_assert_eq!(self.get_clause_from_clauses(0).len(), 0);
            match self.strengthen(k) {
                StrengthenResult::Success => {}
                StrengthenResult::Failure { depth: _ } => {
                    return IC3V2Result::CTX {
                        depth: k.try_into().unwrap(),
                    };
                }
            };
            self.propagate_clauses(k);
            for i in 1..(k + 1) {
                // all clauses in i+1 should be in i.
                debug_assert!(self
                    .get_clause_from_clauses(i + 1)
                    .iter()
                    .all(|c| self.get_clause_from_clauses(i).contains(c)));
                if self.get_clause_from_clauses(i).len()
                    == self.get_clause_from_clauses(i + 1).len()
                {
                    // todo: compare just the lengths
                    self.print_progress_if_verbose(k);
                    return IC3V2Result::Proof {
                        invariant: self.get_fk(i),
                    };
                }
            }
        }
        unreachable!();
    }
}
