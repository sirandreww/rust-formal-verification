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
        formulas::literal::VariableType,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };
    use std::collections::HashMap;

    // ********************************************************************************************
    // macro
    // ********************************************************************************************

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    // ********************************************************************************************
    // Enum
    // ********************************************************************************************

    enum BMCResult {
        NoCTX,
        CTX {
            assignment: HashMap<VariableType, bool>,
            depth: VariableType,
        },
    }

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn bmc(fsts: &FiniteStateTransitionSystem) -> BMCResult {
        let bmc_limit = 10;
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
            AndInverterGraph::from_aig_path("tests/examples/ours/counter_with_bad_assertion.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        let res = bmc(&fsts);
        match res {
            BMCResult::NoCTX => {
                panic!();
            }
            BMCResult::CTX { assignment, depth } => {
                assert_eq!(
                    assignment,
                    hashmap![
                        1 => false, 2 => false, 3 => false, 4 => true, 5 => true,
                        6 => true, 7 => false, 8 => false, 9 => true, 10 => false,
                        11 =>false, 12 => true, 13 => false, 14 => false, 15 => false,
                        16 => false, 17 => false, 18 => true
                    ]
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
            "tests/examples/ours/counter_with_2_bad_assertions.aig",
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
                    hashmap![
                        1 => false, 2 => false, 3 => false, 4 => true, 5 => true,
                        6 => true, 7 => false, 8 => false, 9 => true, 10 => false,
                        11 =>false, 12 => true, 13 => false
                    ]
                );
                assert_eq!(depth, 2);
            }
        }
    }
}
