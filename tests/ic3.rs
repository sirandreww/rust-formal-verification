// ************************************************************************************************
// mod declaration
// ************************************************************************************************

mod common;

// ************************************************************************************************
// test mod declaration
// ************************************************************************************************

#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use std::{collections::{HashMap}, vec, cmp::max};
    use rust_formal_verification::formulas::{Clause, Literal, Cube};
    use rust_formal_verification::{
        formulas::{CNF, literal::VariableType},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rand::rngs::ThreadRng;
    use crate::common;

    // ********************************************************************************************
    // Enum
    // ********************************************************************************************

    pub enum IC3Result {
        Proof { invariant: CNF },
        CTX { depth: VariableType },
    }

    pub enum StrengthenResult {
        Success,
        Failure { depth: VariableType }
    }

    pub enum InductivelyGeneralizeResult {
        Success { n: usize },
        Failure
    }

    // ************************************************************************************************
    // struct
    // ************************************************************************************************

    pub struct IC3 {
        clauses: Vec<CNF>,
        fin_state: FiniteStateTransitionSystem,
        solver: SplrSolver,
        rng: ThreadRng,
    }

    // ************************************************************************************************
    // impl
    // ************************************************************************************************

    impl IC3 {
        pub fn new(fin_state: &FiniteStateTransitionSystem) -> Self {
            Self {
                clauses: Vec::new(),
                fin_state: fin_state.to_owned(),
                solver: SplrSolver::default(),
                rng: thread_rng(),
            }
        }

        fn get_fk(&self, k: usize) -> CNF {
            let mut clauses_fk = self.clauses[k].to_owned();
            if k == 0 {
                clauses_fk.append(&self.fin_state.get_initial_relation());
            } else {
                clauses_fk.append(&self.fin_state.get_safety_property_for_some_depth(0));
            }
            clauses_fk
        }

        fn get_ctx_from_assignment(&self, assignment: &HashMap<VariableType, bool>) -> Vec<Vec<bool>> {
            let mut result = Vec::new();

            let mut latch_literals = self.fin_state.get_input_literal_numbers();
            latch_literals.sort();

            let mut clk = Vec::new();
            for input_lit_num in latch_literals {
                clk.push(assignment[&input_lit_num])
            }
            result.push(clk);
            result
        }

        fn is_bad_reached_in_0_steps(&self) -> SatResponse {
            let mut cnf = CNF::new();
            cnf.append(&self.fin_state.get_initial_relation());
            cnf.append(&self.fin_state.get_unsafety_property_for_some_depth(0));
            self.solver.solve_cnf(&cnf)
        }

        fn is_bad_reached_in_1_steps(&self) -> SatResponse {
            let mut cnf = CNF::new();
            cnf.append(&self.fin_state.get_initial_relation());
            cnf.append(&self.fin_state.get_transition_relation_for_some_depth(1));
            cnf.append(&self.fin_state.get_unsafety_property_for_some_depth(1));
            self.solver.solve_cnf(&cnf)
        }

        fn propagate_clauses(&mut self, k: usize) {
            for i in 1..(k+1) {
                let clauses_fi = self.clauses[i];
                for c in clauses_fi.iter() {
                    let mut cnf = CNF::new();
                    cnf.append(&self.get_fk(i));
                    cnf.append(&self.fin_state.get_transition_relation_for_some_depth(1));
                    cnf.append(&self.fin_state.add_depth_to_property(&(!(c.to_owned())).to_cnf(), 1));
                    match self.solver.solve_cnf(&cnf) {
                        SatResponse::UnSat => {
                            // can propagate this property :)
                            self.clauses[i+1].add_clause(c);
                        },
                        SatResponse::Sat { assignment: _ } => { 
                            // can't propagate this clause :(
                        },
                    }
                }
            }
        }

        fn extract_predecessor_from_assignment(&self, assignment: &HashMap<VariableType, bool>) -> Cube {
            let mut literals = Vec::new();
            let latch_literals = self.fin_state.get_state_literal_numbers();
            
            for state_lit_num in latch_literals {
                literals.push(
                    Literal::new(state_lit_num)
                    .negate_if_true(!assignment[&state_lit_num])
                )
            }

            Cube::new(&literals)
        }

        fn is_bad_reached_in_1_step_from_cnf(&self, cnf: &CNF) -> SatResponse {
            let mut new_cnf = CNF::new();
            new_cnf.append(cnf);
            new_cnf.append(&self.fin_state.get_transition_relation_for_some_depth(1));
            new_cnf.append(&self.fin_state.get_unsafety_property_for_some_depth(1));
            self.solver.solve_cnf(&new_cnf)
        }

        // calculates sat(Fi ^ T ^ !s ^ s')
        fn is_fi_and_t_and_not_s_and_s_tag_sat(&self, i: usize, s: &Cube) -> bool {
            let mut new_cnf = CNF::new();
            new_cnf.append(&self.get_fk(i));
            new_cnf.append(&self.fin_state.get_transition_relation_for_some_depth(1));
            new_cnf.add_clause(&!(s.to_owned()));
            new_cnf.append(&self.fin_state.add_depth_to_property(&s.to_cnf(), 1));
            return match self.solver.solve_cnf(&new_cnf) {
                SatResponse::UnSat => { false },
                SatResponse::Sat { assignment: _ } => { true }
            };
        }

        fn is_clause_inductive_relative_to_fi(&self, d: &Clause, i: usize) -> bool {

        }

        fn get_subclause_of_not_s_that_is_inductive_relative_to_fi(&self, s: &Cube, i: usize) -> Clause {
            // clause generalize(s,i) {
                // c = ¬s = (l1 ⋁ l2 ⋁ … ⋁ lk)
                let c = !(s.to_owned());
                let mut c_literals: Vec<Literal> = c.iter().map(|l| l.to_owned()).collect();
                c_literals.shuffle(&mut self.rng);
                let mut i = 0;
                while i < c_literals.len() {
                    let removed = c_literals.swap_remove(i);
                    let d = Clause::new(&c_literals);
                    if self.is_clause_inductive_relative_to_fi(&d, i){
                        // remove successful
                    } else {
                        // undo remove
                        c_literals.push(removed);
                        c_literals.swap(i, c_literals.len() - 1);
                    }
                }
                Clause::new(&c_literals)
        }

        fn generate_clause(&self, s: &Cube, i: usize, k: usize) {
            let c = self.get_subclause_of_not_s_that_is_inductive_relative_to_fi(s, i);
            for j in 1..(i+2) {
                self.clauses[j].add_clause(&c);
            }
        }

        fn inductively_generalize(&self, s: &Cube, min: usize, k: usize) -> InductivelyGeneralizeResult {
            if min < 0 && self.is_fi_and_t_and_not_s_and_s_tag_sat(0, s){
                return InductivelyGeneralizeResult::Failure;
            }

            for i in max(1, min + 1)..(k + 1){
                if self.is_fi_and_t_and_not_s_and_s_tag_sat(i, s) {
                    self.generate_clause(s, i - 1, k);
                    return InductivelyGeneralizeResult::Success { n: i - 1 };
                }
            }
            self.generate_clause(s, k, k);
            InductivelyGeneralizeResult::Success { n: k }
        }

        fn push_generalization(&self, queue , k) {

        }

        fn strengthen(&mut self, k: usize) -> StrengthenResult {
            loop {
                match self.is_bad_reached_in_1_step_from_cnf(&self.get_fk(k)) {
                    SatResponse::UnSat => {
                        break;
                    },
                    SatResponse::Sat { assignment } => {
                        let s = self.extract_predecessor_from_assignment(&assignment);
                        let n = self.inductively_generalize(s, k - 2, k);
                        self.push_generalization({(n+1, s)}, k);
                    },
                }
            }

            StrengthenResult::Success
        }

        pub fn prove(&mut self) -> IC3Result {
            let init_and_not_p = self.is_bad_reached_in_0_steps();
            match init_and_not_p {
                SatResponse::Sat { assignment: _ } => return IC3Result::CTX { depth: 0 },
                SatResponse::UnSat => (),
            }
    
            let init_and_tr_and_not_p_tag = self.is_bad_reached_in_1_steps();
            match init_and_tr_and_not_p_tag {
                SatResponse::Sat { assignment: _ } => return IC3Result::CTX { depth: 1 },
                SatResponse::UnSat => (),
            }
    
    
            for k in 1.. {
                debug_assert!(self.clauses.len() == (k + 1));
                match self.strengthen(k) {
                    StrengthenResult::Success => {},
                    StrengthenResult::Failure { depth } => { return IC3Result::CTX { depth }; },
                };
                self.propagate_clauses(k);
                for i in 1..(k+1) {
                    if self.clauses[i] == self.clauses[i+1] {
                        return IC3Result::Proof { invariant: self.get_fk(i) };
                    }
                }
            }
            unreachable!();
        }
    }

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn ic3(
        fin_state: &FiniteStateTransitionSystem,
        _aig: &AndInverterGraph,
    ) -> IC3Result {
        let mut ic3_solver = IC3::new(fin_state);
        return ic3_solver.prove();
    }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    #[test]
    fn pdr_on_2020_examples() {
        let file_paths = common::_get_paths_to_all_aig_for_2020();
        for aig_file_path in file_paths {
            println!("file_path = {}", aig_file_path);

            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            ic3(&fin_state, &aig);
        }
    }
}
