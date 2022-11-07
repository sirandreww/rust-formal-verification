// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::AIGNodeType;
use crate::models::and_inverter_graph::AndInverterGraph;
use std::collections::HashMap;

// ************************************************************************************************
// AIG simulation result
// ************************************************************************************************

pub struct AIGSimulationResult {
    states: Vec<Vec<bool>>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn get_pre_initial_simulation_state(
        &self,
        initial_input: &HashMap<usize, bool>,
        initial_latches: &HashMap<usize, bool>,
    ) -> Vec<bool> {
        let mut result = vec![false; self.nodes.len()];
        for (input, value) in initial_input.iter() {
            result[input.to_owned()] = value.to_owned();
        }
        for (latch, value) in initial_latches.iter() {
            let node = &self.nodes[latch.to_owned()];
            let latch_input = node.get_latch_input();
            if latch_input % 2 == 0 {
                result[latch_input >> 1] = value.to_owned();
            } else {
                result[latch_input >> 1] = !value.to_owned();
            }
        }
        assert_eq!(result.len(), self.nodes.len());
        result
    }

    fn get_next_simulation_state(
        &self,
        last_state: &[bool],
        current_input: &HashMap<usize, bool>,
    ) -> Vec<bool> {
        let mut result = Vec::new();
        for aig_node in self.nodes.iter() {
            let var_num = aig_node.get_literal() >> 1;
            assert_eq!(result.len(), var_num);
            match aig_node.get_type() {
                AIGNodeType::ConstantZero => {
                    assert_eq!(result.len(), 0);
                    result.push(false);
                }
                AIGNodeType::Input => {
                    result.push(current_input[&var_num]);
                }
                AIGNodeType::Latch => {
                    let latch_in = aig_node.get_latch_input();
                    result.push(if latch_in % 2 == 0 {
                        last_state[&latch_in >> 1]
                    } else {
                        !last_state[&latch_in >> 1]
                    });
                }
                AIGNodeType::And => {
                    let rhs0 = aig_node.get_and_rhs0();
                    let rhs1 = aig_node.get_and_rhs1();
                    let rhs0_value = if rhs0 % 2 == 0 {
                        result[rhs0 >> 1]
                    } else {
                        !result[rhs0 >> 1]
                    };
                    let rhs1_value = if rhs1 % 2 == 0 {
                        result[rhs1 >> 1]
                    } else {
                        !result[rhs1 >> 1]
                    };
                    result.push(rhs0_value && rhs1_value);
                }
            }
        }
        assert_eq!(result.len(), self.nodes.len());
        result
    }

    // ********************************************************************************************
    // aig getting and gates
    // ********************************************************************************************

    pub fn simulate(
        &self,
        inputs: &Vec<HashMap<usize, bool>>,
        initial_latches: &HashMap<usize, bool>,
    ) -> AIGSimulationResult {
        // check inputs
        assert!(
            !inputs.is_empty(),
            "Inputs cannot be empty to start simulation."
        );
        for clk_inputs in inputs {
            assert_eq!(clk_inputs.len(), self.number_of_inputs);
            assert_eq!(self.inputs.len(), self.number_of_inputs);
            // check that each clock has the correct var numbers.
            for input_var in self.inputs.iter() {
                assert!(clk_inputs.contains_key(input_var));
            }
        }
        // check initial latches
        assert!(
            initial_latches.len() <= self.number_of_latches,
            "Too many initial latches provided."
        );
        assert_eq!(self.latches.len(), self.number_of_latches);
        for latch_var in self.latches.iter() {
            assert!(initial_latches.contains_key(&latch_var));
        }

        // prepare result
        let mut result = AIGSimulationResult { states: Vec::new() };

        let mut last_state = self.get_pre_initial_simulation_state(&inputs[0], initial_latches);
        println!("State -1 = {:?}", last_state);
        for clk_number in 0..inputs.len() {
            let current_input = &inputs[clk_number];
            let next_state = self.get_next_simulation_state(&last_state, current_input);
            last_state = next_state.to_owned();
            result.states.push(next_state);
            println!("State {} = {:?}", clk_number, last_state);
        }
        result
    }
}
