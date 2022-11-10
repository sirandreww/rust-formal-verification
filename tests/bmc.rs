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

    use crate::common;
    use rust_formal_verification::{
        algorithms::{bmc::BMCResult, BMC},
        formulas::literal::VariableType,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{Assignment, SplrSolver},
    };
    use std::{
        collections::HashMap,
        time::{Duration, Instant},
    };

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn extract_inputs_from_assignment(
        assignment: &Assignment,
        fin_state: &FiniteStateTransitionSystem,
        number_of_clocks_wanted: VariableType,
    ) -> Vec<HashMap<usize, bool>> {
        let mut result = Vec::new();
        let input_literals = fin_state.get_input_literal_numbers();
        for _ in 0..number_of_clocks_wanted {
            let mut clk_inputs = HashMap::new(); // <usize, bool>
            for input in input_literals.iter() {
                let val = assignment.get_value_of_variable(input);
                let input: usize = input.to_owned().try_into().unwrap();
                clk_inputs.insert(input, val);
            }
            result.push(clk_inputs);
        }
        result
    }

    fn extract_initial_latches_from_assignment(
        assignment: &Assignment,
        fin_state: &FiniteStateTransitionSystem,
    ) -> HashMap<usize, bool> {
        let mut result = HashMap::new();
        let state_literals = fin_state.get_state_literal_numbers();

        for state in state_literals {
            let val = assignment.get_value_of_variable(&state);
            let input: usize = state.to_owned().try_into().unwrap();
            result.insert(input, val);
        }

        result
    }

    fn check_that_bad_is_true_only_for_last_cycle(
        aig: &AndInverterGraph,
        sim_res: &Vec<Vec<bool>>,
    ) {
        // get bad wires
        let bad_literals = aig.get_bad_information();

        // go over all clk cycles
        for (i, state) in sim_res.iter().enumerate() {
            // keep track of if you saw a bad wire that had a true value
            let mut was_bad_seen = false;
            // go over all bad wires
            for bad_lit in bad_literals.iter() {
                // get value
                let bad_variable = bad_lit >> 1;
                let wire_value = if bad_lit % 2 == 0 {
                    state[bad_variable]
                } else {
                    !state[bad_variable]
                };

                // update if bad was seen
                was_bad_seen = was_bad_seen || wire_value;
            }
            if i == (sim_res.len() - 1) {
                // last cycle
                assert!(was_bad_seen);
            } else {
                // not last cycle
                assert!(!was_bad_seen);
            }
        }
    }

    fn bmc_test(
        aig_path: &str,
        expected_assignment: &Option<Assignment>,
        expected_depth: Option<u32>,
        timeout_in_seconds: u64,
        search_depth_limit: u32,
        is_ctx_certain: bool,
        is_known_to_be_safe: bool,
    ) -> bool {
        println!("{}", aig_path);
        let start = Instant::now();
        let aig = AndInverterGraph::from_aig_path(aig_path);
        let fin_state = FiniteStateTransitionSystem::from_aig(&aig);

        let bmc = BMC::<SplrSolver>::new(true);
        let res = bmc.search(
            &fin_state,
            search_depth_limit,
            Duration::from_secs(timeout_in_seconds),
        );
        match res {
            BMCResult::NoCTX { depth_reached } => {
                if is_ctx_certain {
                    panic!("CTX is certain yet none was found...");
                }

                println!(
                    "Seems Ok, ran till depth = {}\t, time = {}",
                    depth_reached,
                    start.elapsed().as_secs_f32()
                );

                false
            }
            BMCResult::CTX { assignment, depth } => {
                match expected_assignment {
                    Some(eas) => assert_eq!(&assignment, eas),
                    None => {}
                };
                match expected_depth {
                    Some(ed) => assert_eq!(depth, ed),
                    None => {}
                };

                let inputs = extract_inputs_from_assignment(&assignment, &fin_state, depth + 1);
                let initial_latches =
                    extract_initial_latches_from_assignment(&assignment, &fin_state);
                assert_eq!(inputs.len() - 1, depth.try_into().unwrap());

                let sim_result = aig.simulate(&inputs, &initial_latches);
                assert_eq!(sim_result.len() - 1, depth.try_into().unwrap());
                check_that_bad_is_true_only_for_last_cycle(&aig, &sim_result);

                if is_known_to_be_safe {
                    // How did we get here anyway? previous checks should have caught this...
                    panic!("Test is known to be safe but BMC found counter example. How did we get here anyway? previous checks should have caught this...")
                }

                println!(
                    "UNSAFE - CTX found at depth = {}, time = {}",
                    depth,
                    start.elapsed().as_secs_f32()
                );

                true
            }
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
        bmc_test(
            "tests/examples/ours/counter_with_bad_assertion.aig",
            &Some(Assignment::from_dimacs_assignment(&[
                -1, -2, -3, 4, 5, 6, -7, -8, 9, -10, -11, 12, -13, -14, -15, -16, -17, 18,
            ])),
            Some(3),
            5,
            10,
            true,
            false,
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
    //      |_________| BAD |           |  and  ) x4---> |______/
    //                      |    *---> |______/
    // *--------------------*    |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn bmc_on_simple_example_counter_with_2_bad_assertion() {
        bmc_test(
            "tests/examples/ours/counter_with_2_bad_assertions.aig",
            &Some(Assignment::from_dimacs_assignment(&[
                -1, -2, -3, 4, 5, 6, -7, -8, 9, -10, -11, 12, -13,
            ])),
            Some(2),
            5,
            10,
            true,
            false,
        );
    }

    #[test]
    fn bmc_on_hwmcc20_only_unconstrained_problems() {
        let run_test = true;
        let probability_of_test = 0.05;
        let mut number_of_solved = 0;
        if run_test {
            let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
            for aig_file_path in file_paths {
                if common::_true_with_probability(probability_of_test) {
                    let solved = bmc_test(&aig_file_path, &None, None, 10, 20, false, false);
                    number_of_solved += if solved { 1 } else { 0 };
                }
            }
        }
        println!("Number of solved problems = {}", number_of_solved);
    }
}
