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

    use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};

    fn test_bmc(
        aig_path: &str,
        expected_init: &str,
        expected_trans: &str,
        expected_safe: &str,
        expected_unsafe: &str,
    ) {
        let aig = AndInverterGraph::from_aig_path(aig_path);
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);
        let bmc_limit = 50;
        for depth in 1..bmc_limit {
            let transition_unrolled = fsts.unroll_transition_relation(depth);
            let bad_after_steps = fsts.get_unsafety_property_after_unrolling(depth);
        }
    }

    // ********************************************************************************************
    // creating fsts test
    // ********************************************************************************************


    #[test]
    fn create_fsts_from_simple_example_counter_with_bad_assertion() {
        // test_fsts_creation(
        //     "tests/simple_examples/counter_with_bad_assertion.aig",
        //     "((!x1) & (!x2) & (!x3))",
        //     "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))",
        //     "((!x3))",
        //     "((x3))"
        // )
    }

    #[test]
    fn create_fsts_from_simple_example_counter_with_2_bad_assertions() {
        // test_fsts_creation(
        //     "tests/simple_examples/counter_with_2_bad_assertions.aig",
        //     "((!x1) & (!x2) & (!x3))",
        //     "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))",
        //     "((!x2) & (!x3))",
        //     "((x2 | x3))"
        // )
    }
}
