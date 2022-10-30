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

    use rand::Rng;
    use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
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
        let aig = AndInverterGraph::from_aig_path("tests/simple_examples/counter.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "((!x1) & (!x2) & (!x3))"
        );
        assert_eq!(fsts.get_safety_property_for_some_depth(0).to_string(), "()");
        assert_eq!(
            fsts.get_unsafety_property_for_some_depth(0).to_string(),
            "()"
        );
        assert_eq!(
            fsts.get_transition_relation_for_some_depth(1).to_string(),
            "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))"
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
            AndInverterGraph::from_aig_path("tests/simple_examples/counter_with_bad_assertion.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "((!x1) & (!x2) & (!x3))"
        );
        assert_eq!(
            fsts.get_safety_property_for_some_depth(0).to_string(),
            "((!x3))"
        );
        assert_eq!(
            fsts.get_unsafety_property_for_some_depth(0).to_string(),
            "((x3))"
        );
        assert_eq!(
            fsts.get_transition_relation_for_some_depth(1).to_string(),
            "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))"
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
            "tests/simple_examples/counter_with_2_bad_assertions.aig",
        );
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        assert_eq!(
            fsts.get_initial_relation().to_string(),
            "((!x1) & (!x2) & (!x3))"
        );
        assert_eq!(
            fsts.get_safety_property_for_some_depth(0).to_string(),
            "((!x2) & (!x3))"
        );
        assert_eq!(
            fsts.get_unsafety_property_for_some_depth(0).to_string(),
            "((x2 | x3))"
        );
        assert_eq!(
            fsts.get_transition_relation_for_some_depth(1).to_string(),
            "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))"
        );
    }

    #[test]
    fn read_all_aig_files_from_hwmcc20() {
        let file_paths = common::get_paths_to_all_aig_and_corresponding_aag_files();
        for (aig_file_path, _aag_file_path) in file_paths {
            let mut rng = rand::thread_rng();
            let random_number_between_0_and_1: f64 = rng.gen();
            // make the test faster by only doing this with 5% of the files
            if random_number_between_0_and_1 > 0.95 {
                println!("file_path = {}", aig_file_path);
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                let _fsts = FiniteStateTransitionSystem::from_aig(&aig);
            }
        }
    }
}
