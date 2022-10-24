// ************************************************************************************************
// mod declaration
// ************************************************************************************************

// mod common;

// ************************************************************************************************
// test mod declaration
// ************************************************************************************************

#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use rust_formal_verification::{
        formulas::CNF,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };

    // ********************************************************************************************
    // creating fsts test
    // ********************************************************************************************

    // *--------------------------------------------------------------*
    // |     _________                                                |
    // |    |         |                                               |
    // *--> | latch 0 | x1-*                                          |
    //      |_________|    |                                          |
    //                     |                                          |
    // *-------------------*-------------Not-------*                  |
    // |     _________                             |     ______       |
    // |    |         |                 ______     *--> |      \      |
    // *--> | latch 1 | x2-*---Not---> |      \         |  and  ) x5--*
    //      |_________|    |           |  and  ) x4---> |______/
    //                     |     *---> |______/
    // *-------------------*     |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn bmc_on_simple_example_counter_with_bad_assertion() {
        let aig =
            AndInverterGraph::from_aig_path("tests/simple_examples/counter_with_bad_assertion.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);
        let bmc_limit: u32 = 10;
        for depth in 0..bmc_limit {
            let mut cnf_to_check = CNF::default();
            fsts.get_initial_relation(&mut cnf_to_check);
            for unroll_depth in 1..(depth + 1) {
                fsts.get_transition_relation_for_some_depth(unroll_depth, &mut cnf_to_check);
            }
            fsts.get_unsafety_property_for_some_depth(depth, &mut cnf_to_check);
            let solver = SplrSolver::default();
            let response = solver.solve_cnf(&cnf_to_check);
            match response {
                SatResponse::Sat { assignment } => {
                    assert_eq!(assignment, vec![-1, -2, -3, 4, 5, 6, -7, -8, 9, -10, -11, 12, -13, -14, -15, -16, -17, 18, -19, -20]);
                    assert!(depth == 3);
                    return;
                }
                SatResponse::UnSat => {
                    assert!(depth == 0 || depth == 1 || depth == 2);
                }
            };
        }
    }

    // *--------------------------------------------------------------*
    // |     _________                                                |
    // |    |         |                                               |
    // *--> | latch 0 | x1-*                                          |
    //      |_________|    |                                          |
    //                     |                                          |
    // *-------------------*-------------Not-------*                  |
    // |     _________                             |     ______       |
    // |    |         |                 ______     *--> |      \      |
    // *--> | latch 1 | x2--*--Not---> |      \         |  and  ) x5--*
    //      |_________| BAD |           |  and  ) x4---> |______/
    //                      |    *---> |______/
    // *--------------------*    |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn bmc_on_simple_example_counter_with_2_bad_assertion() {
        let aig =
            AndInverterGraph::from_aig_path("tests/simple_examples/counter_with_2_bad_assertions.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);
        let bmc_limit: u32 = 10;
        for depth in 0..bmc_limit {
            let mut cnf_to_check = CNF::default();
            fsts.get_initial_relation(&mut cnf_to_check);
            for unroll_depth in 1..(depth + 1) {
                fsts.get_transition_relation_for_some_depth(unroll_depth, &mut cnf_to_check);
            }
            fsts.get_unsafety_property_for_some_depth(depth, &mut cnf_to_check);
            let solver = SplrSolver::default();
            let response = solver.solve_cnf(&cnf_to_check);
            match response {
                SatResponse::Sat { assignment } => {
                    assert_eq!(assignment, vec![-1, -2, -3, 4, 5, 6, -7, -8, 9, -10, -11, 12, -13, -14, -15]);
                    assert!(depth == 2);
                    return;
                }
                SatResponse::UnSat => {
                    assert!(depth == 0 || depth == 1);
                }
            };
        }
    }
}
