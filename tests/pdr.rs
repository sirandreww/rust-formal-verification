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

    use rust_formal_verification::{
        formulas::{CNF, Cube, Clause},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };

    use crate::common;

    // ********************************************************************************************
    // Enum
    // ********************************************************************************************

    enum PdrResult {
        Proof { invariant: CNF },
        CTX { cube: Cube }
    }

    fn get_bad_cube(fin_state: &FiniteStateTransitionSystem, blocked_cubes_of_each_frame: Vec<Vec<Cube>>, unreachable_cubes: Vec<Cube> ) {

    }

    fn is_bad_reached_in_0_steps(fin_state: &FiniteStateTransitionSystem) -> SatResponse{
        let mut cnf = CNF::default();
        fin_state.get_initial_relation(&mut cnf);
        fin_state.get_unsafety_property_for_some_depth(0, &mut cnf);
        let solver = SplrSolver::default();
        let response = solver.solve_cnf(&cnf);
        response
    }

    fn is_bad_reached_in_1_steps(fin_state: &FiniteStateTransitionSystem) -> SatResponse{
        let mut cnf = CNF::default();
        fin_state.get_initial_relation(&mut cnf);
        fin_state.get_transition_relation_for_some_depth(1, &mut cnf);
        fin_state.get_unsafety_property_for_some_depth(1, &mut cnf);
        let solver = SplrSolver::default();
        let response = solver.solve_cnf(&cnf);
        response
    }

    fn is_bad_reached_in_1_step_from_f_k(f_k: &CNF, fin_state: &FiniteStateTransitionSystem) -> SatResponse{
        let mut cnf = CNF::default();
        for c in f_k.iter() {
            cnf.add_clause(c);
        }
        fin_state.get_transition_relation_for_some_depth(1, &mut cnf);
        fin_state.get_unsafety_property_for_some_depth(1, &mut cnf);
        let solver = SplrSolver::default();
        let response = solver.solve_cnf(&cnf);
        response
    }

    // fn shrink_cube_using_trinary_sim(cube: Cube) -> Cube {

    // }

    fn block(cube_to_block: Cube, frame_to_block_in: i32){

    }

    fn generalize(cube: Cube, i :i32) -> Clause {
        let mut clause = !cube;
        // let mut potential_clause_update = !cube;
        // clause.
        // for(j=1; j<=k; j++) {
        //     d = c \ {lj};
        //     if (!(Init ∧ ¬d) && !((Fi ∧ d)∧ Tr ∧ ¬d’))
        //     c = d;
        // }
        return clause;
    }

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn property_directed_reachability(fin_state: &FiniteStateTransitionSystem, aig: &AndInverterGraph) -> PdrResult{

        let init_and_not_p = is_bad_reached_in_0_steps(fin_state);
        match init_and_not_p {
            SatResponse::Sat { cube } => return PdrResult::CTX { cube },
            SatResponse::UnSat => (),
        }

        let init_and_tr_and_not_p_tag = is_bad_reached_in_1_steps(fin_state);
        match init_and_tr_and_not_p_tag {
            SatResponse::Sat { cube } => return PdrResult::CTX { cube },
            SatResponse::UnSat => (),
        }

        let mut f0 = CNF::default();
        fin_state.get_initial_relation(&mut f0);

        let mut f1 = CNF::default();
        fin_state.get_safety_property_for_some_depth(0, &mut f1);

        let mut F = vec![f0, f1];

        for k in 1.. {
            loop {
                let fk_and_tr_and_not_p_tag = is_bad_reached_in_1_step_from_f_k(F.last().unwrap(), fin_state);
                match fk_and_tr_and_not_p_tag {
                    SatResponse::Sat { cube } => {
                        block(cube,k-1)
                    },
                    SatResponse::UnSat => { break; },
                }
            }
            let mut p = CNF::default();
            fin_state.get_safety_property_for_some_depth(0, &mut p);
            F.push(p);
        }

        unreachable!();
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
