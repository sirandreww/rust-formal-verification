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

    fn is_bad_reached_in_0_steps(fin_state: &FiniteStateTransitionSystem) -> SatResponse {
        let mut cnf = CNF::new();
        cnf.append(&fin_state.get_initial_relation());
        cnf.append(&fin_state.get_unsafety_property_for_some_depth(0));

        let solver = SplrSolver::default();
        solver.solve_cnf(&cnf)
    }

    fn is_bad_reached_in_1_steps(fin_state: &FiniteStateTransitionSystem) -> SatResponse {
        let mut cnf = CNF::new();
        cnf.append(&fin_state.get_initial_relation());
        cnf.append(&fin_state.get_transition_relation_for_some_depth(1));
        cnf.append(&fin_state.get_unsafety_property_for_some_depth(1));

        let solver = SplrSolver::default();
        solver.solve_cnf(&cnf)
    }

    fn is_bad_reached_in_1_step_from_cnf(
        cnf: &CNF,
        fin_state: &FiniteStateTransitionSystem,
    ) -> SatResponse {
        let mut new_cnf = CNF::new();
        new_cnf.append(&cnf.to_owned());
        new_cnf.append(&fin_state.get_transition_relation_for_some_depth(1));
        new_cnf.append(&fin_state.get_unsafety_property_for_some_depth(1));

        let solver = SplrSolver::default();
        solver.solve_cnf(cnf)
    }

    // fn shrink_cube_using_trinary_sim(cube: Cube) -> Cube {

    // }

    // fn block(cube_to_block: Cube, frame_to_block_in: i32){

    // }

    // fn extract_state_from_assignment(fin_state: &FiniteStateTransitionSystem, assignment: Vec<i32>) -> Vec<bool> {
    //     fin_state.convert_assignment_to_input_and_state(assignment);

    // }

    // fn generalize(cube: Cube, i :i32) -> Clause {
    //     let mut clause = !cube;
    //     // let mut potential_clause_update = !cube;
    //     // clause.
    //     // for(j=1; j<=k; j++) {
    //     //     d = c \ {lj};
    //     //     if (!(Init ∧ ¬d) && !((Fi ∧ d)∧ Tr ∧ ¬d’))
    //     //     c = d;
    //     // }
    //     return clause;
    // }

    fn get_ctx_from_assignment(fin_state: &FiniteStateTransitionSystem, assignment: Vec<i32>) -> Vec<Vec<bool>> {
        let mut result = Vec::new();
        let input_literals: HashSet<u32> = fin_state.get_input_literals().iter().map(|l| l.get_number()).collect();
        let max_variable_number: usize = fin_state.get_max_literal_number().try_into().unwrap();
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

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn property_directed_reachability(
        fin_state: &FiniteStateTransitionSystem,
        _aig: &AndInverterGraph,
    ) -> IC3Result {
        let init_and_not_p = is_bad_reached_in_0_steps(fin_state);
        match init_and_not_p {
            SatResponse::Sat { assignment } => return IC3Result::CTX { input: vec![] },
            SatResponse::UnSat => (),
        }

        let init_and_tr_and_not_p_tag = is_bad_reached_in_1_steps(fin_state);
        match init_and_tr_and_not_p_tag {
            SatResponse::Sat { assignment } => return IC3Result::CTX {
                input:get_ctx_from_assignment(fin_state, assignment) 
            },
            SatResponse::UnSat => (),
        }

        let mut F = vec![
            fin_state.get_initial_relation(),
            fin_state.get_safety_property_for_some_depth(0),
        ];
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
            property_directed_reachability(&fin_state, &aig);
        }
    }
}
