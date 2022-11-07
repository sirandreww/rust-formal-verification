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
    };
    use std::{collections::HashMap, time::Instant};

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
    // helper functions
    // ********************************************************************************************

    fn extract_inputs_from_assignment(
        assignment: &HashMap<VariableType, bool>,
        fin_state: &FiniteStateTransitionSystem,
    ) -> Vec<HashMap<usize, bool>> {
        let mut result = Vec::new();
        let input_literals = fin_state.get_input_literal_numbers();
        let max_literal: VariableType = fin_state.get_max_literal_number().try_into().unwrap();
        let assignment_length: VariableType = assignment.len().try_into().unwrap();
        let number_of_clocks_in_assignment = (assignment_length + max_literal + 1) / (max_literal);
        for _ in 0..(number_of_clocks_in_assignment - 1) {
            let mut clk_inputs = HashMap::new(); // <usize, bool>
            for input in input_literals.iter() {
                let val = if assignment.contains_key(input) {
                    // doesn't matter
                    false
                } else {
                    assignment.get(input).unwrap().to_owned()
                };
                let input: usize = input.to_owned().try_into().unwrap();
                clk_inputs.insert(input, val);
            }
            result.push(clk_inputs);
        }
        result
    }

    fn extract_initial_latches_from_assignment(
        assignment: &HashMap<VariableType, bool>,
        fin_state: &FiniteStateTransitionSystem,
    ) -> HashMap<usize, bool> {
        let mut result = HashMap::new();
        let state_literals = fin_state.get_state_literal_numbers();

        for state in state_literals {
            let val = if assignment.contains_key(&state) {
                // doesn't matter
                false
            } else {
                assignment.get(&state).unwrap().to_owned()
            };
            let input: usize = state.to_owned().try_into().unwrap();
            result.insert(input, val);
        }

        result
    }

    fn bmc_test(aig_path: &str, expected_assignment: &HashMap<u32, bool>, expected_depth: u32) {
        let aig = AndInverterGraph::from_aig_path(aig_path);
        let fin_state = FiniteStateTransitionSystem::from_aig(&aig);

        let bmc = BMC::new();
        let res = bmc.search(5, &fin_state);
        match res {
            BMCResult::NoCTX { depth_reached: _ } => {
                panic!();
            }
            BMCResult::CTX { assignment, depth } => {
                assert_eq!(&assignment, expected_assignment);
                assert_eq!(depth, expected_depth);

                let inputs = extract_inputs_from_assignment(&assignment, &fin_state);
                let initial_latches =
                    extract_initial_latches_from_assignment(&assignment, &fin_state);
                assert_eq!(inputs.len(), depth.try_into().unwrap());

                let _sim_result = aig.simulate(&inputs, &initial_latches);
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
            &hashmap![
                1 => false, 2 => false, 3 => false, 4 => true, 5 => true,
                6 => true, 7 => false, 8 => false, 9 => true, 10 => false,
                11 =>false, 12 => true, 13 => false, 14 => false, 15 => false,
                16 => false, 17 => false, 18 => true
            ],
            3,
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
            &hashmap![
                1 => false, 2 => false, 3 => false, 4 => true, 5 => true,
                6 => true, 7 => false, 8 => false, 9 => true, 10 => false,
                11 =>false, 12 => true, 13 => false
            ],
            2,
        );
    }

    #[test]
    fn bmc_on_hwmcc20_only_unconstrained_problems() {
        let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
        for aig_file_path in file_paths {
            println!("{}", aig_file_path);
            let start = Instant::now();
            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            let bmc = BMC::new();
            let res = bmc.search(5, &fin_state);
            match res {
                BMCResult::NoCTX { depth_reached } => {
                    println!(
                        "Seems Ok, ran till depth = {}\t, time = {}",
                        depth_reached,
                        start.elapsed().as_secs_f32()
                    );
                }
                BMCResult::CTX {
                    assignment: _,
                    depth,
                } => {
                    println!(
                        "UNSAFE - CTX found at depth = {}, time = {}",
                        depth,
                        start.elapsed().as_secs_f32()
                    );
                }
            }
        }
    }
}
