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
    fn create_fsts_from_simple_example_counter_with_bad_assertion() {
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
                    println!("{:?}", assignment);
                    assert!(depth == 4);
                }
                SatResponse::UnSat => {
                    assert!(depth == 0 || depth == 1 || depth == 3);
                }
            };
        }
        // let mut initial_got = CNF::default();
        // fsts.get_initial_relation(&mut initial_got);

        // let mut initial_expected = CNF::default();

        // let x0 = Literal::new(&Variable::new(0));
        // let x1 = Literal::new(&Variable::new(1));
        // let x2 = Literal::new(&Variable::new(2));
        // let x3 = Literal::new(&Variable::new(3));
        // let x4 = Literal::new(&Variable::new(4));
        // let x5 = Literal::new(&Variable::new(5));

        // initial_expected.add_clause(&Clause::new(&[!x0]));
        // initial_expected.add_clause(&Clause::new(&[!x1]));
        // initial_expected.add_clause(&Clause::new(&[!x2]));
        // initial_expected.add_clause(&Clause::new(&[!x3]));

        // initial_expected.add_clause(&Clause::new(&[!x4, !x2]));
        // initial_expected.add_clause(&Clause::new(&[!x4, !x3]));
        // initial_expected.add_clause(&Clause::new(&[x4, x2, x3]));

        // initial_expected.add_clause(&Clause::new(&[!x5, x4]));
        // initial_expected.add_clause(&Clause::new(&[!x5, !x1]));
        // initial_expected.add_clause(&Clause::new(&[x5, !x4, x1]));

        // assert_eq!(initial_got.to_string(), initial_expected.to_string());

        // // safety means !x3
        // let mut safety_on_the_literals = CNF::default();
        // fsts.get_safety_property_for_some_depth(0, &mut safety_on_the_literals);
        // let mut expected_safety = CNF::default();
        // expected_safety.add_clause(&Clause::new(&[!x3]));
        // assert_eq!(safety_on_the_literals.to_string(), expected_safety.to_string());

        // // unsafety is empty
        // let mut unsafety_on_the_literals = CNF::default();
        // fsts.get_unsafety_property_for_some_depth(0, &mut unsafety_on_the_literals);
        // let mut expected_unsafety = CNF::default();
        // expected_unsafety.add_clause(&Clause::new(&[x3]));
        // assert_eq!(unsafety_on_the_literals.to_string(), expected_unsafety.to_string());

        // let x6 = Literal::new(&Variable::new(6));
        // let x7 = Literal::new(&Variable::new(7));
        // let x8 = Literal::new(&Variable::new(8));
        // let x9 = Literal::new(&Variable::new(9));
        // let x10 = Literal::new(&Variable::new(10));

        // // try transition
        // // x7 = x1
        // initial_expected.add_clause(&Clause::new(&[!x1, x7]));
        // initial_expected.add_clause(&Clause::new(&[x1, !x7]));

        // // x8 = x2
        // initial_expected.add_clause(&Clause::new(&[!x2, x8]));
        // initial_expected.add_clause(&Clause::new(&[x2, !x8]));

        // // x6 = x5
        // initial_expected.add_clause(&Clause::new(&[!x5, x6]));
        // initial_expected.add_clause(&Clause::new(&[x5, !x6]));

        // // x9 = !x7 & !x8
        // initial_expected.add_clause(&Clause::new(&[!x9, !x7]));
        // initial_expected.add_clause(&Clause::new(&[!x9, !x8]));
        // initial_expected.add_clause(&Clause::new(&[x9, x7, x8]));

        // // x10 = x9 ^ !x6
        // initial_expected.add_clause(&Clause::new(&[!x10, x9]));
        // initial_expected.add_clause(&Clause::new(&[!x10, !x6]));
        // initial_expected.add_clause(&Clause::new(&[x10, !x9, x6]));

        // fsts.get_transition_relation_for_some_depth(1, &mut initial_got);
        // assert_eq!(initial_got.to_string(), initial_expected.to_string());
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
