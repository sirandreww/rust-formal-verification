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

    use rust_formal_verification::{models::{AndInverterGraph, FiniteStateTransitionSystem}, algorithms::formula_logic::is_a_and_b_satisfiable, solvers::sat::VarisatSolver};
    // use std::fs;

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

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
    // *--> | latch 2 | x3-------*
    //      |_________|
    #[test]
    fn create_fsts_from_simple_example_counter() {
        let aig = AndInverterGraph::from_aig_path("tests/examples/ours/counter.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "((!x1) & (!x2) & (!x3))"
        );
        assert_eq!(fsts.get_safety_property().to_string(), "()"); // empty CNF is always true.
        assert_eq!(
            fsts.get_unsafety_property().to_string(),
            "(())" // a cnf with the empty clause is simply always false.
        );
        assert_eq!(
            fsts.get_transition_relation().to_string(),
            "((x1 | !x7) & (!x1 | !x5) & (!x1 | x7) & (x2 | !x8) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (x4 | !x5) & (x5 | !x6) & (!x5 | x6) & (x1 | !x4 | x5) & (x2 | x3 | x4))"
        );
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
    // *--> | latch 1 | x2-*---Not---> |      \         |  and  ) x5--*
    //      |_________|    |           |  and  ) x4---> |______/
    //                     |     *---> |______/
    // *-------------------*     |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn create_fsts_from_simple_example_counter_with_bad_assertion() {
        let aig =
            AndInverterGraph::from_aig_path("tests/examples/ours/counter_with_bad_assertion.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "((!x1) & (!x2) & (!x3))"
        );
        assert_eq!(
            fsts.get_safety_property().to_string(),
            "((!x3))"
        );
        assert_eq!(
            fsts.get_unsafety_property().to_string(),
            "((x3))"
        );
        assert_eq!(
            fsts.get_transition_relation().to_string(),
            "((x1 | !x7) & (!x1 | !x5) & (!x1 | x7) & (x2 | !x8) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (x4 | !x5) & (x5 | !x6) & (!x5 | x6) & (x1 | !x4 | x5) & (x2 | x3 | x4))"
        );
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
    //      |_________| BAD |          |  and  ) x4---> |______/
    //                      |    *---> |______/
    // *--------------------*    |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn create_fsts_from_simple_example_counter_with_2_bad_assertions() {
        let aig = AndInverterGraph::from_aig_path(
            "tests/examples/ours/counter_with_2_bad_assertions.aig",
        );
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "((!x1) & (!x2) & (!x3))"
        );
        assert_eq!(
            fsts.get_safety_property().to_string(),
            "((!x2) & (!x3))"
        );
        assert_eq!(
            fsts.get_unsafety_property().to_string(),
            "((x2 | x3))"
        );
        assert_eq!(
            fsts.get_transition_relation().to_string(),
            "((x1 | !x7) & (!x1 | !x5) & (!x1 | x7) & (x2 | !x8) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (x4 | !x5) & (x5 | !x6) & (!x5 | x6) & (x1 | !x4 | x5) & (x2 | x3 | x4))"
        );
    }

    #[test]
    fn read_all_aig_files_from_hwmcc20() {
        let depth_to_test_for = 3;

        let file_paths = common::_get_paths_to_all_aig_and_corresponding_aag_files();
        for (aig_file_path, _) in file_paths {
            // make the test faster by only doing this with 5% of the files
            if common::_true_with_probability(0.05) {
                println!("file_path = {}", aig_file_path);
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                let fsts = FiniteStateTransitionSystem::from_aig(&aig);
                
                // check that p and not !p cannot hold at the same time.
                for depth in 0..depth_to_test_for{
                    assert!(
                        !is_a_and_b_satisfiable::<VarisatSolver>(
                            &fsts.add_tags_to_relation(&fsts.get_safety_property(), depth),
                            &fsts.add_tags_to_relation(&fsts.get_unsafety_property(), depth)
                        )
                    );
                }
            }
        }
    }
}
