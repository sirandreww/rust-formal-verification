// ********************************************************************************************
// use
// ********************************************************************************************

use std::{ cmp::min, collections::BinaryHeap};

use rand::{rngs::ThreadRng, thread_rng};

use crate::{models::FiniteStateTransitionSystem, solvers::sat::SatSolver, formulas::{CNF, literal::VariableType, Cube}};


// ********************************************************************************************
// Enum
// ********************************************************************************************

pub enum PDRResult {
    Proof { invariant: CNF },
    CTX { depth: VariableType },
}

pub enum Frame {
    NULL,
    INF,
    Ok(usize)
}

pub struct TCube {
    cube: Cube,
    frame: Frame
}

impl Ord for TCube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct PDR<T: SatSolver> {
    f: Vec<Vec<CNF>>,
    fin_state: FiniteStateTransitionSystem,
    solver: T,
    rng: ThreadRng,
    latch_literals: Vec<u32>,
    _input_literals: Vec<u32>,
    // caching for speedup
    initial: CNF,
    transition: CNF,
    p0: CNF,
    not_p0: CNF,
    not_p1: CNF,
    // for printing
    verbose: bool,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl<T: SatSolver> PDR<T> {

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    // fn get_fk(&self, k: usize) -> CNF {
    //     let mut clauses_fk = self.clauses[k].to_owned();
    //     if k == 0 {
    //         clauses_fk.append(&self.initial);
    //     } else {
    //         clauses_fk.append(&self.p0);
    //     }
    //     clauses_fk
    // }

    // fn _get_ctx_from_assignment(&self, assignment: &HashMap<VariableType, bool>) -> Vec<Vec<bool>> {
    //     let mut result = Vec::new();

    //     let mut clk = Vec::new();
    //     for input_lit_num in self.input_literals.iter() {
    //         clk.push(assignment[&input_lit_num])
    //     }
    //     result.push(clk);
    //     result
    // }

    // fn is_bad_reached_in_0_steps(&self) -> SatResponse {
    //     let mut cnf = CNF::new();
    //     cnf.append(&self.initial);
    //     cnf.append(&self.not_p0);
    //     // println!("I ^ !P = {}", cnf);
    //     self.solver.solve_cnf(&cnf)
    // }

    // fn is_bad_reached_in_1_steps(&self) -> SatResponse {
    //     let mut cnf = CNF::new();
    //     cnf.append(&self.initial);
    //     cnf.append(&self.transition);
    //     cnf.append(&self.not_p1);
    //     // println!("I ^ T ^ !P' = {}", cnf);
    //     self.solver.solve_cnf(&cnf)
    // }

    // fn propagate_clauses(&mut self, k: usize) {
    //     for i in 1..(k + 1) {
    //         let clauses_fi = self.clauses[i].to_owned();
    //         for c in clauses_fi.iter() {
    //             let mut cnf = CNF::new();
    //             cnf.append(&self.get_fk(i));
    //             cnf.append(&self.transition);
    //             cnf.append(
    //                 &self
    //                     .fin_state
    //                     .add_tags_to_relation(&(!(c.to_owned())).to_cnf(), 1),
    //             );
    //             match self.solver.solve_cnf(&cnf) {
    //                 SatResponse::UnSat => {
    //                     // can propagate this property :)
    //                     self.clauses[i + 1].add_clause(c);
    //                 }
    //                 SatResponse::Sat { assignment: _ } => {
    //                     // can't propagate this clause :(
    //                 }
    //             }
    //         }
    //     }
    // }

    // fn extract_predecessor_from_assignment(&self, assignment: &Assignment) -> Cube {
    //     let mut literals = Vec::new();

    //     for state_lit_num in &self.latch_literals {
    //         literals.push(
    //             Literal::new(state_lit_num.to_owned())
    //                 .negate_if_true(!assignment.get_value_of_variable(state_lit_num)),
    //         )
    //     }

    //     Cube::new(&literals)
    // }

    // fn is_bad_reached_in_1_step_from_cnf(&self, cnf: &CNF) -> SatResponse {
    //     let mut new_cnf = CNF::new();
    //     new_cnf.append(cnf);
    //     new_cnf.append(&self.transition);
    //     new_cnf.append(&self.not_p1);
    //     self.solver.solve_cnf(&new_cnf)
    // }

    // calculates sat(Fi ^ T ^ !s ^ s')
    // fn is_fi_and_t_and_not_s_and_s_tag_sat(&self, i: usize, s: &Cube) -> bool {
    //     let mut new_cnf = CNF::new();
    //     new_cnf.append(&self.get_fk(i));
    //     new_cnf.append(&self.transition);
    //     new_cnf.add_clause(&!(s.to_owned()));
    //     new_cnf.append(&self.fin_state.add_tags_to_relation(&s.to_cnf(), 1));
    //     match self.solver.solve_cnf(&new_cnf) {
    //         SatResponse::UnSat => false,
    //         SatResponse::Sat { assignment: _ } => true,
    //     }
    // }

    // fn is_clause_inductive_relative_to_fi(&self, d: &Clause, i: usize) -> bool {
    //     // return !(Init ∧ ¬d) && !((Fi ∧ d)∧ Tr ∧ ¬d’)

    //     let mut first_cnf = self.initial.to_owned();
    //     first_cnf.append(&(!d.to_owned()).to_cnf());
    //     match self.solver.solve_cnf(&first_cnf) {
    //         SatResponse::UnSat => {}
    //         SatResponse::Sat { assignment: _ } => {
    //             return false;
    //         }
    //     }

    //     let mut second_cnf = self.get_fk(i);
    //     second_cnf.add_clause(d);
    //     second_cnf.append(&self.transition);
    //     second_cnf.append(
    //         &self
    //             .fin_state
    //             .add_tags_to_relation(&(!d.to_owned()).to_cnf(), 1),
    //     );
    //     match self.solver.solve_cnf(&second_cnf) {
    //         SatResponse::UnSat => true,
    //         SatResponse::Sat { assignment: _ } => false,
    //     }
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
    //     for j in 1..(i + 2) {
    //         self.clauses[j].add_clause(&c);
    //     }
    // }

    // fn inductively_generalize(
    //     &mut self,
    //     s: &Cube,
    //     min: isize,
    //     k: usize,
    // ) -> InductivelyGeneralizeResult {
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

    // // calculates sat(Fi ^ T ^ s')
    // fn solve_fi_and_t_and_s_tag(&self, i: usize, s: &Cube) -> SatResponse {
    //     let mut new_cnf = CNF::new();
    //     new_cnf.append(&self.get_fk(i));
    //     new_cnf.append(&self.transition);
    //     new_cnf.append(&self.fin_state.add_tags_to_relation(&s.to_cnf(), 1));
    //     self.solver.solve_cnf(&new_cnf)
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
    //         match self.solve_fi_and_t_and_s_tag(n, &s) {
    //             SatResponse::Sat { assignment } => {
    //                 // we have to block p in order to block n.
    //                 let p = self.extract_predecessor_from_assignment(&assignment);
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

    // fn print_progress(&self, k: usize) {
    //     println!(
    //         "IC3 is on k = {}, clauses lengths = {:?}",
    //         k,
    //         self.clauses.iter().map(|c| c.len()).collect::<Vec<usize>>()
    //     );
    // }

    // fn strengthen(&mut self, k: usize) -> StrengthenResult {
    //     loop {
    //         match self.is_bad_reached_in_1_step_from_cnf(&self.get_fk(k)) {
    //             SatResponse::UnSat => {
    //                 break;
    //             }
    //             SatResponse::Sat { assignment } => {
    //                 let s = self.extract_predecessor_from_assignment(&assignment);
    //                 // println!("Should block s = {} from F{}", s, k - 1);
    //                 match self.inductively_generalize(
    //                     &s,
    //                     <usize as TryInto<isize>>::try_into(k).unwrap() - 2,
    //                     k,
    //                 ) {
    //                     InductivelyGeneralizeResult::Failure => {
    //                         return StrengthenResult::Failure {
    //                             depth: k.try_into().unwrap(),
    //                         };
    //                     }
    //                     InductivelyGeneralizeResult::Success { n } => {
    //                         let mut queue = PriorityQueue::<Cube, Reverse<usize>>::new();
    //                         queue.push(s, Reverse(n + 1));
    //                         match self.push_generalization(&queue, k) {
    //                             PushGeneralizeResult::Failure => {
    //                                 return StrengthenResult::Failure {
    //                                     depth: k.try_into().unwrap(),
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

    fn get_bad_cube(&self) -> Option<Cube>{

    }

    fn depth(&self) -> usize {
        self.f.len() - 2
    }

    fn new_frame(&mut self){
        let n = self.f.len();
        self.f.push(Vec::new());
        self.f.swap(n - 1, n);
    }

    fn cond_assign(s: &mut TCube, t: TCube) -> bool {
        match t.frame {
            Frame::NULL => {
                s = t;
                true
            },
            _ => {false}
        }
    }

    // adds a cube to Fa nd the PdrSat object. It will also remove any subsumed cube in F.
    // Subsumed cube in the Sat-Solver will be removed through periodic recycling.???????
    fn add_blocked_cube(&self, s: &TCube) {
        match s.frame {
            Frame::Ok(s_frame_depth) => {
                let k = min(s_frame_depth, self.depth() + 1);
                
                // remove subsumed clauses
                for d in 1..(k + 1){
                    let mut i = 0;
                    while i < self.f[d].len(){
                        if self.subsumes(s.cube, self.f[d][i]) {

                        } else {
                            i += 1;
                        }
                    }
                }
            },
            _ => { unreachable!(); }
        }
    }

    fn rec_block_cube(s0: &TCube) -> bool {
        let q = BinaryHeap::<TCube>::new();
        q.push(s0);
        while q.len() > 0 {
            let s = q.pop().unwrap();
            match s.frame{
                Frame::Ok(s_frame) => {
                    
                },
                _ => {unreachable!();}
            }
        }

    }

    // ********************************************************************************************
    // API functions
    // ********************************************************************************************

    pub fn new(fin_state: &FiniteStateTransitionSystem, verbose: bool) -> Self {
        let mut p0 = fin_state.get_state_to_properties_relation();
        p0.append(&fin_state.get_safety_property());

        let mut not_p0 = fin_state.get_state_to_properties_relation();
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

    pub fn pdr_main(&mut self) -> PDRResult{
        self.f.push(Vec::new()); // push F_inf
        self.new_frame(); // create "F[0]"

        loop {
            let c = self.get_bad_cube();
            match c {
                Some(c_cube) => {
                    if ! self.rec_block_cube(TCube{ cube: c_cube, frame: Frame::Ok(self.depth()) }){
                        // failed to block 'c' => CTX found
                        return PDRResult::CTX { depth: self.depth().try_into().unwrap() };
                    }
                },
                None => {
                    self.new_frame();
                    if self.propagate_blocked_cubes(){
                        return PDRResult::Proof { invariant: 
                            self.get_invariant()
                        }
                    }
                },
            }
        }
    }
}
