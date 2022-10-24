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

    use rust_formal_verification::{models::{AndInverterGraph, FiniteStateTransitionSystem}, formulas::{CNF, Clause, Literal, Variable}};
    // use std::fs;

    // use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************


    // fn test_fsts_creation(
    //     aig_path: &str,
    //     expected_init: &str,
    //     expected_trans: &str,
    //     expected_safe: &str,
    //     expected_unsafe: &str,
    //     expected_trans_unrolled: &str,
    //     expected_unsafe_unrolled_1: &str,
    //     expected_unsafe_unrolled_2: &str,
    // ) {
    //     let aig = AndInverterGraph::from_aig_path(aig_path);
    //     let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    //     // assert_eq!(fsts.get_initial_states().to_string(), expected_init);
    //     // assert_eq!(fsts.get_transition_formula().to_string(), expected_trans);
    //     // assert_eq!(fsts.get_safety_property().to_string(), expected_safe);
    //     // assert_eq!(fsts.get_unsafety_property().to_string(), expected_unsafe);
    //     // assert_eq!(
    //     //     fsts.unroll_transition_relation(1).to_string(),
    //     //     expected_trans
    //     // );
    //     // assert_eq!(
    //     //     fsts.unroll_transition_relation(2).to_string(),
    //     //     expected_trans_unrolled
    //     // );
    //     // assert_eq!(
    //     //     fsts.get_unsafety_property_after_unrolling(1).to_string(),
    //     //     expected_unsafe_unrolled_1
    //     // );
    //     // assert_eq!(
    //     //     fsts.get_unsafety_property_after_unrolling(2).to_string(),
    //     //     expected_unsafe_unrolled_2
    //     // );
    // }

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
        let mut initial_got = CNF::default();
        fsts.get_initial_relation(&mut initial_got);

        let mut initial_expected = CNF::default();

        let x0 = Literal::new(&Variable::new(0));
        let x1 = Literal::new(&Variable::new(1));
        let x2 = Literal::new(&Variable::new(2));
        let x3 = Literal::new(&Variable::new(3));
        let x4 = Literal::new(&Variable::new(4));
        let x5 = Literal::new(&Variable::new(5));

        initial_expected.add_clause(&Clause::new(&[!x0]));
        initial_expected.add_clause(&Clause::new(&[!x1]));
        initial_expected.add_clause(&Clause::new(&[!x2]));
        initial_expected.add_clause(&Clause::new(&[!x3]));

        initial_expected.add_clause(&Clause::new(&[!x4, !x2]));
        initial_expected.add_clause(&Clause::new(&[!x4, !x3]));
        initial_expected.add_clause(&Clause::new(&[x4, x2, x3]));

        initial_expected.add_clause(&Clause::new(&[!x5, x4]));
        initial_expected.add_clause(&Clause::new(&[!x5, !x1]));
        initial_expected.add_clause(&Clause::new(&[x5, !x4, x1]));

        assert_eq!(initial_got.to_string(), initial_expected.to_string());

        // safety is empty
        let mut safety_on_the_literals = CNF::default();
        fsts.get_safety_property_for_some_depth(0, &mut safety_on_the_literals);
        assert_eq!(safety_on_the_literals.to_string(), CNF::default().to_string());

        // unsafety is empty
        let mut unsafety_on_the_literals = CNF::default();
        fsts.get_unsafety_property_for_some_depth(0, &mut unsafety_on_the_literals);
        assert_eq!(unsafety_on_the_literals.to_string(), CNF::default().to_string());

        let x6 = Literal::new(&Variable::new(6));
        let x7 = Literal::new(&Variable::new(7));
        let x8 = Literal::new(&Variable::new(8));
        let x9 = Literal::new(&Variable::new(9));
        let x10 = Literal::new(&Variable::new(10));

        // try transition
        // x7 = x1
        initial_expected.add_clause(&Clause::new(&[!x1, x7]));
        initial_expected.add_clause(&Clause::new(&[x1, !x7]));

        // x8 = x2
        initial_expected.add_clause(&Clause::new(&[!x2, x8]));
        initial_expected.add_clause(&Clause::new(&[x2, !x8]));

        // x6 = x5
        initial_expected.add_clause(&Clause::new(&[!x5, x6]));
        initial_expected.add_clause(&Clause::new(&[x5, !x6]));

        // x9 = !x7 & !x8
        initial_expected.add_clause(&Clause::new(&[!x9, !x7]));
        initial_expected.add_clause(&Clause::new(&[!x9, !x8]));
        initial_expected.add_clause(&Clause::new(&[x9, x7, x8]));

        // x10 = x9 ^ !x6
        initial_expected.add_clause(&Clause::new(&[!x10, x9]));
        initial_expected.add_clause(&Clause::new(&[!x10, !x6]));
        initial_expected.add_clause(&Clause::new(&[x10, !x9, x6]));

        fsts.get_transition_relation_for_some_depth(1, &mut initial_got);
        assert_eq!(initial_got.to_string(), initial_expected.to_string());

    }

    #[test]
    fn create_fsts_from_simple_example_counter_with_bad_assertion() {
        let aig = AndInverterGraph::from_aig_path("tests/simple_examples/counter_with_bad_assertion.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);
        let mut initial_got = CNF::default();
        fsts.get_initial_relation(&mut initial_got);

        let mut initial_expected = CNF::default();

        let x0 = Literal::new(&Variable::new(0));
        let x1 = Literal::new(&Variable::new(1));
        let x2 = Literal::new(&Variable::new(2));
        let x3 = Literal::new(&Variable::new(3));
        let x4 = Literal::new(&Variable::new(4));
        let x5 = Literal::new(&Variable::new(5));

        initial_expected.add_clause(&Clause::new(&[!x0]));
        initial_expected.add_clause(&Clause::new(&[!x1]));
        initial_expected.add_clause(&Clause::new(&[!x2]));
        initial_expected.add_clause(&Clause::new(&[!x3]));

        initial_expected.add_clause(&Clause::new(&[!x4, !x2]));
        initial_expected.add_clause(&Clause::new(&[!x4, !x3]));
        initial_expected.add_clause(&Clause::new(&[x4, x2, x3]));

        initial_expected.add_clause(&Clause::new(&[!x5, x4]));
        initial_expected.add_clause(&Clause::new(&[!x5, !x1]));
        initial_expected.add_clause(&Clause::new(&[x5, !x4, x1]));

        assert_eq!(initial_got.to_string(), initial_expected.to_string());

        // safety means !x3
        let mut safety_on_the_literals = CNF::default();
        fsts.get_safety_property_for_some_depth(0, &mut safety_on_the_literals);
        let mut expected_safety = CNF::default();
        expected_safety.add_clause(&Clause::new(&[!x3]));
        assert_eq!(safety_on_the_literals.to_string(), expected_safety.to_string());

        // unsafety is empty
        let mut unsafety_on_the_literals = CNF::default();
        fsts.get_unsafety_property_for_some_depth(0, &mut unsafety_on_the_literals);
        let mut expected_unsafety = CNF::default();
        expected_unsafety.add_clause(&Clause::new(&[x3]));
        assert_eq!(unsafety_on_the_literals.to_string(), expected_unsafety.to_string());

        let x6 = Literal::new(&Variable::new(6));
        let x7 = Literal::new(&Variable::new(7));
        let x8 = Literal::new(&Variable::new(8));
        let x9 = Literal::new(&Variable::new(9));
        let x10 = Literal::new(&Variable::new(10));

        // try transition
        // x7 = x1
        initial_expected.add_clause(&Clause::new(&[!x1, x7]));
        initial_expected.add_clause(&Clause::new(&[x1, !x7]));

        // x8 = x2
        initial_expected.add_clause(&Clause::new(&[!x2, x8]));
        initial_expected.add_clause(&Clause::new(&[x2, !x8]));

        // x6 = x5
        initial_expected.add_clause(&Clause::new(&[!x5, x6]));
        initial_expected.add_clause(&Clause::new(&[x5, !x6]));

        // x9 = !x7 & !x8
        initial_expected.add_clause(&Clause::new(&[!x9, !x7]));
        initial_expected.add_clause(&Clause::new(&[!x9, !x8]));
        initial_expected.add_clause(&Clause::new(&[x9, x7, x8]));

        // x10 = x9 ^ !x6
        initial_expected.add_clause(&Clause::new(&[!x10, x9]));
        initial_expected.add_clause(&Clause::new(&[!x10, !x6]));
        initial_expected.add_clause(&Clause::new(&[x10, !x9, x6]));

        fsts.get_transition_relation_for_some_depth(1, &mut initial_got);
        assert_eq!(initial_got.to_string(), initial_expected.to_string());
    }

    #[test]
    fn create_fsts_from_simple_example_counter_with_2_bad_assertions() {
        let aig = AndInverterGraph::from_aig_path("tests/simple_examples/counter_with_2_bad_assertions.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);
        let mut initial_got = CNF::default();
        fsts.get_initial_relation(&mut initial_got);

        let mut initial_expected = CNF::default();

        let x0 = Literal::new(&Variable::new(0));
        let x1 = Literal::new(&Variable::new(1));
        let x2 = Literal::new(&Variable::new(2));
        let x3 = Literal::new(&Variable::new(3));
        let x4 = Literal::new(&Variable::new(4));
        let x5 = Literal::new(&Variable::new(5));

        initial_expected.add_clause(&Clause::new(&[!x0]));
        initial_expected.add_clause(&Clause::new(&[!x1]));
        initial_expected.add_clause(&Clause::new(&[!x2]));
        initial_expected.add_clause(&Clause::new(&[!x3]));

        initial_expected.add_clause(&Clause::new(&[!x4, !x2]));
        initial_expected.add_clause(&Clause::new(&[!x4, !x3]));
        initial_expected.add_clause(&Clause::new(&[x4, x2, x3]));

        initial_expected.add_clause(&Clause::new(&[!x5, x4]));
        initial_expected.add_clause(&Clause::new(&[!x5, !x1]));
        initial_expected.add_clause(&Clause::new(&[x5, !x4, x1]));

        assert_eq!(initial_got.to_string(), initial_expected.to_string());

        // safety means !x3
        let mut safety_on_the_literals = CNF::default();
        fsts.get_safety_property_for_some_depth(0, &mut safety_on_the_literals);
        let mut expected_safety = CNF::default();
        expected_safety.add_clause(&Clause::new(&[!x3]));
        expected_safety.add_clause(&Clause::new(&[!x2]));
        assert_eq!(safety_on_the_literals.to_string(), expected_safety.to_string());

        // unsafety is empty
        let mut unsafety_on_the_literals = CNF::default();
        fsts.get_unsafety_property_for_some_depth(0, &mut unsafety_on_the_literals);
        let mut expected_unsafety = CNF::default();
        expected_unsafety.add_clause(&Clause::new(&[x3, x2]));
        assert_eq!(unsafety_on_the_literals.to_string(), expected_unsafety.to_string());

        let x6 = Literal::new(&Variable::new(6));
        let x7 = Literal::new(&Variable::new(7));
        let x8 = Literal::new(&Variable::new(8));
        let x9 = Literal::new(&Variable::new(9));
        let x10 = Literal::new(&Variable::new(10));

        // try transition
        // x7 = x1
        initial_expected.add_clause(&Clause::new(&[!x1, x7]));
        initial_expected.add_clause(&Clause::new(&[x1, !x7]));

        // x8 = x2
        initial_expected.add_clause(&Clause::new(&[!x2, x8]));
        initial_expected.add_clause(&Clause::new(&[x2, !x8]));

        // x6 = x5
        initial_expected.add_clause(&Clause::new(&[!x5, x6]));
        initial_expected.add_clause(&Clause::new(&[x5, !x6]));

        // x9 = !x7 & !x8
        initial_expected.add_clause(&Clause::new(&[!x9, !x7]));
        initial_expected.add_clause(&Clause::new(&[!x9, !x8]));
        initial_expected.add_clause(&Clause::new(&[x9, x7, x8]));

        // x10 = x9 ^ !x6
        initial_expected.add_clause(&Clause::new(&[!x10, x9]));
        initial_expected.add_clause(&Clause::new(&[!x10, !x6]));
        initial_expected.add_clause(&Clause::new(&[x10, !x9, x6]));

        fsts.get_transition_relation_for_some_depth(1, &mut initial_got);
        assert_eq!(initial_got.to_string(), initial_expected.to_string());
    }
}
