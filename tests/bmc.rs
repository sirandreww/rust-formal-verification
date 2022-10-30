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
    // Enum
    // ********************************************************************************************

    enum BMCResult {
        NoCTX,
        CTX { assignment: Vec<i32>, depth: u32 },
    }

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn bmc(fsts: &FiniteStateTransitionSystem) -> BMCResult {
        let bmc_limit: u32 = 10;
        for depth in 0..bmc_limit {
            let mut sat_formula = fsts.get_initial_relation();
            for unroll_depth in 1..(depth + 1) {
                sat_formula.append(&fsts.get_transition_relation_for_some_depth(unroll_depth));
            }
            sat_formula.append(&fsts.get_unsafety_property_for_some_depth(depth));

            let solver = SplrSolver::default();
            let response = solver.solve_cnf(&sat_formula);
            match response {
                SatResponse::Sat { assignment } => {
                    return BMCResult::CTX {
                        assignment: assignment,
                        depth: depth,
                    }
                }
                SatResponse::UnSat => {}
            }
        }
        return BMCResult::NoCTX;
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

        let res = bmc(&fsts);
        match res {
            BMCResult::NoCTX => {
                panic!();
            }
            BMCResult::CTX { assignment, depth } => {
                assert_eq!(
                    assignment,
                    vec![-1, -2, -3, 4, 5, 6, -7, -8, 9, -10, -11, 12, -13, -14, -15, -16, -17, 18]
                );
                assert_eq!(depth, 3);
            }
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
        let aig = AndInverterGraph::from_aig_path(
            "tests/simple_examples/counter_with_2_bad_assertions.aig",
        );
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        let res = bmc(&fsts);
        match res {
            BMCResult::NoCTX => {
                panic!();
            }
            BMCResult::CTX { assignment, depth } => {
                assert_eq!(
                    assignment,
                    vec![-1, -2, -3, 4, 5, 6, -7, -8, 9, -10, -11, 12, -13]
                );
                assert_eq!(depth, 2);
            }
        }
    }
}
