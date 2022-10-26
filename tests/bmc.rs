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
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn bmc(fsts: &FiniteStateTransitionSystem) {
        let bmc_limit: u32 = 10;
        for depth in 0..bmc_limit {
            let mut sat_formula = fsts.get_initial_relation();
            for unroll_depth in 1..(depth + 1) {
                sat_formula.append(fsts.get_transition_relation_for_some_depth(unroll_depth));
            }
            sat_formula.append(fsts.get_unsafety_property_for_some_depth(depth));

            let solver = SplrSolver::default();
            let response = solver.solve_cnf(&sat_formula);
            match response {
                SatResponse::Sat { assignment } => {
                    assert_eq!(assignment, vec![]);
                    assert!(depth == 3);
                    return;
                }
                SatResponse::UnSat => {
                    assert!(depth == 0 || depth == 1 || depth == 2);
                }
            };
        }
    }

    // ********************************************************************************************
    // tests
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

        bmc(&fsts);
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
        let aig = AndInverterGraph::from_aig_path(
            "tests/simple_examples/counter_with_2_bad_assertions.aig",
        );
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        bmc(&fsts);
    }
}
