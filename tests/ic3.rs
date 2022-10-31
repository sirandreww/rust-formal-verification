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

    use std::collections::HashSet;

    use rust_formal_verification::{
        formulas::CNF,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };

    use crate::common;

    // ********************************************************************************************
    // Enum
    // ********************************************************************************************

    enum IC3Result {
        Proof { invariant: CNF },
        CTX { input: Vec<Vec<bool>> },
    }

    // ************************************************************************************************
    // struct
    // ************************************************************************************************

    pub struct IC3 {
        F: Vec<CNF>,
        fin_state: FiniteStateTransitionSystem
    }

    // ************************************************************************************************
    // impl
    // ************************************************************************************************

    impl IC3 {
        pub fn new(fin_state: &FiniteStateTransitionSystem) -> Self {
            Self { 
                F: Vec::new(),
                fin_state: fin_state.to_owned()
            }
        }

        fn get_ctx_from_assignment(&self, assignment: Vec<i32>) -> Vec<Vec<bool>> {
            let mut result = Vec::new();
            let input_literals: HashSet<u32> = self.fin_state.get_input_literals().iter().map(|l| l.get_number()).collect();
            let max_variable_number: usize = self.fin_state.get_max_literal_number().try_into().unwrap();
            let mut this_clk = Vec::new();
            for (i_usize, var_value) in assignment.iter().enumerate() {
                let i: u32 = i_usize.try_into().unwrap(); 
                assert!(var_value.abs() == (i + 1).try_into().unwrap());
                if i_usize % max_variable_number == 1 {
                    // new clk
                    result.push(this_clk);
                    this_clk = Vec::new();                
                }
                if input_literals.contains(&i) {
                    this_clk.push(var_value.is_positive());
                }
            }
            result.push(this_clk);
            result
        }

        fn is_bad_reached_in_0_steps(&self) -> SatResponse {
            let mut cnf = CNF::new();
            cnf.append(&self.fin_state.get_initial_relation());
            cnf.append(&self.fin_state.get_unsafety_property_for_some_depth(0));
    
            let solver = SplrSolver::default();
            solver.solve_cnf(&cnf)
        }

        fn is_bad_reached_in_1_steps(&self) -> SatResponse {
            let mut cnf = CNF::new();
            cnf.append(&self.fin_state.get_initial_relation());
            cnf.append(&self.fin_state.get_transition_relation_for_some_depth(1));
            cnf.append(&self.fin_state.get_unsafety_property_for_some_depth(1));
    
            let solver = SplrSolver::default();
            solver.solve_cnf(&cnf)
        }

        fn propagateClauses(k: i32) {
            for i in 1..k {
                
            }
        }

        pub fn prove(&mut self) -> IC3Result {
            let init_and_not_p = self.is_bad_reached_in_0_steps();
            match init_and_not_p {
                SatResponse::Sat { assignment } => return IC3Result::CTX { input: vec![] },
                SatResponse::UnSat => (),
            }
    
            let init_and_tr_and_not_p_tag = self.is_bad_reached_in_1_steps();
            match init_and_tr_and_not_p_tag {
                SatResponse::Sat { assignment } => return IC3Result::CTX {
                    input: self.get_ctx_from_assignment(assignment)
                },
                SatResponse::UnSat => (),
            }
    
            self.F = vec![
                self.fin_state.get_initial_relation(),
                self.fin_state.get_safety_property_for_some_depth(0),
            ];
    
            for k in 1.. {
                match self.strengthen(k) {
                    SatResponse::Sat { assignment } => {
                        return IC3Result::CTX {
                                input: self.get_ctx_from_assignment(assignment)
                            }
                        },
                    SatResponse::UnSat => {},
                }
                self.
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
        
    }

    // fn property_directed_reachability(fin_state: &FiniteStateTransitionSystem) -> PdrResult{
    //     let unreachable_cubes = Vec::new();
    // }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    #[test]
    fn pdr_on_2020_examples() {
        let file_paths = common::get_paths_to_all_aig_for_2020();
        for aig_file_path in file_paths {
            println!("file_path = {}", aig_file_path);

            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            ic3(&fin_state, &aig);
        }
    }
}
