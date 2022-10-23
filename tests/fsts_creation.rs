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
    // use std::fs;

    // use crate::common;

    fn test_fsts_creation(
        aig_path: &str,
        expected_init: &str,
        expected_trans: &str,
        expected_safe: &str,
        expected_unsafe: &str,
    ) {
        let aig = AndInverterGraph::from_aig_path(aig_path);
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);
        assert_eq!(fsts.get_initial_states().to_string(), expected_init);
        assert_eq!(fsts.get_transition_formula().to_string(), expected_trans);
        assert_eq!(fsts.get_safety_property().to_string(), expected_safe);
        assert_eq!(fsts.get_unsafety_property().to_string(), expected_unsafe);
    }

    // ********************************************************************************************
    // creating fsts test
    // ********************************************************************************************

    #[test]
    fn create_fsts_from_simple_example_counter() {
        test_fsts_creation(
            "tests/simple_examples/counter.aig",
            "((!x1) & (!x2) & (!x3))",
            "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))",
            "()",
            "()"
        )
    }

    #[test]
    fn create_fsts_from_simple_example_counter_with_bad_assertion() {
        test_fsts_creation(
            "tests/simple_examples/counter_with_bad_assertion.aig",
            "((!x1) & (!x2) & (!x3))",
            "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))",
            "((!x3))",
            "((x3))"
        )
    }

    #[test]
    fn create_fsts_from_simple_example_counter_with_2_bad_assertions() {
        test_fsts_creation(
            "tests/simple_examples/counter_with_2_bad_assertions.aig",
            "((!x1) & (!x2) & (!x3))",
            "((!x1 | !x5) & (!x1 | x7) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (!x5 | x6) & (x1 | !x4 | x5) & (x1 | !x7) & (x2 | !x8) & (x2 | x3 | x4) & (x4 | !x5) & (x5 | !x6))",
            "((!x2) & (!x3))",
            "((x2 | x3))"
        )
    }
}
