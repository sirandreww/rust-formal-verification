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
        algorithms::formula_logic::is_a_and_b_satisfiable,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::stateless::VarisatSolver,
    };
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
        let fsts = FiniteStateTransitionSystem::from_aig(&aig, false);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "p cnf 3 3\n-1 0\n-2 0\n-3 0"
        );
        assert_eq!(fsts.get_safety_property().to_string(), "p cnf 0 0\n"); // empty CNF is always true.
        assert_eq!(
            fsts.get_unsafety_property().to_string(),
            "p cnf 0 1\n 0" // a cnf with the empty clause is simply always false.
        );
        assert_eq!(
            fsts.get_transition_relation().to_string(),
            "p cnf 8 12\n1 -7 0\n-1 -5 0\n-1 7 0\n2 -8 0\n-2 -4 0\n-2 8 0\n-3 -4 0\n4 -5 0\n5 -6 0\n-5 6 0\n1 -4 5 0\n2 3 4 0"
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
        let fsts = FiniteStateTransitionSystem::from_aig(&aig, false);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "p cnf 3 3\n-1 0\n-2 0\n-3 0"
        );
        assert_eq!(fsts.get_safety_property().to_string(), "p cnf 3 1\n-3 0");
        assert_eq!(fsts.get_unsafety_property().to_string(), "p cnf 3 1\n3 0");
        assert_eq!(
            fsts.get_transition_relation().to_string(),
            "p cnf 8 12\n1 -7 0\n-1 -5 0\n-1 7 0\n2 -8 0\n-2 -4 0\n-2 8 0\n-3 -4 0\n4 -5 0\n5 -6 0\n-5 6 0\n1 -4 5 0\n2 3 4 0"
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
        let fsts = FiniteStateTransitionSystem::from_aig(&aig, false);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "p cnf 3 3\n-1 0\n-2 0\n-3 0"
        );
        assert_eq!(
            fsts.get_safety_property().to_string(),
            "p cnf 3 2\n-2 0\n-3 0"
        );
        assert_eq!(fsts.get_unsafety_property().to_string(), "p cnf 3 1\n2 3 0");
        assert_eq!(
            fsts.get_transition_relation().to_string(),
            "p cnf 8 12\n1 -7 0\n-1 -5 0\n-1 7 0\n2 -8 0\n-2 -4 0\n-2 8 0\n-3 -4 0\n4 -5 0\n5 -6 0\n-5 6 0\n1 -4 5 0\n2 3 4 0"
        );
    }

    #[test]
    fn read_all_aig_files_from_hwmcc20() {
        let depth_to_test_for = 3;
        let probability_of_testing_file = 0.05;

        let file_paths = common::_get_paths_to_all_aig_and_corresponding_aag_files();
        for (aig_file_path, _) in file_paths {
            // make the test faster by only doing this with 5% of the files
            if common::_true_with_probability(probability_of_testing_file) {
                println!("file_path = {}", aig_file_path);
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);

                // from_aig only supports unconstrained problems since constraint ones can be folded into unconstrained ones.
                if aig.get_constraints_information().is_empty() {
                    let assume_output_is_bad = aig_file_path.contains("_fold");
                    let fsts = FiniteStateTransitionSystem::from_aig(&aig, assume_output_is_bad);
                    // check that p and not !p cannot hold at the same time for some depths.
                    for depth in 0..depth_to_test_for {
                        assert!(!is_a_and_b_satisfiable::<VarisatSolver>(
                            &fsts.add_tags_to_relation(&fsts.get_safety_property(), depth),
                            &fsts.add_tags_to_relation(&fsts.get_unsafety_property(), depth)
                        ));
                    }
                }
            }
        }
    }
}
