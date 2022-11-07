// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::AIGNodeType;
use crate::models::and_inverter_graph::AndInverterGraph;
use std::collections::HashMap;

use super::aig_node::AIGNode;

// ************************************************************************************************
// AIG simulation result
// ************************************************************************************************

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn get_and_result(aig_node: &AIGNode, current_result: &[bool]) -> bool {
        let rhs0 = aig_node.get_and_rhs0();
        let rhs1 = aig_node.get_and_rhs1();
        let rhs0_value = if rhs0 % 2 == 0 {
            current_result[rhs0 >> 1]
        } else {
            !current_result[rhs0 >> 1]
        };
        let rhs1_value = if rhs1 % 2 == 0 {
            current_result[rhs1 >> 1]
        } else {
            !current_result[rhs1 >> 1]
        };
        rhs0_value && rhs1_value
    }

    fn get_initial_simulation_state(
        &self,
        initial_input: &HashMap<usize, bool>,
        initial_latches: &HashMap<usize, bool>,
    ) -> Vec<bool> {
        let mut result = Vec::new();
        for (i, aig_node) in self.nodes.iter().enumerate() {
            let var_num = aig_node.get_literal() >> 1;
            assert_eq!(var_num, i);
            assert_eq!(result.len(), var_num);
            match aig_node.get_type() {
                AIGNodeType::ConstantZero => {
                    assert_eq!(result.len(), 0);
                    result.push(false);
                }
                AIGNodeType::Input => {
                    result.push(initial_input[&var_num]);
                }
                AIGNodeType::Latch => {
                    let reset = aig_node.get_latch_reset();
                    if reset == 1 {
                        result.push(true);
                    } else if reset == 0 {
                        result.push(false);
                    } else {
                        assert_eq!(reset >> 1, var_num);
                        assert!(
                            initial_latches.contains_key(&var_num),
                            "Initial latch value is unknown, it must be provided."
                        );
                        result.push(initial_latches[&var_num]);
                    }
                }
                AIGNodeType::And => {
                    result.push(Self::get_and_result(aig_node, &result));
                }
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
                    result.push(Self::get_and_result(aig_node, &result));
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
    ) -> Vec<Vec<bool>> {
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
            let node = &self.nodes[latch_var.to_owned()];
            // check that all uninitialized latches have a provided value.
            if &node.get_latch_reset() == latch_var {
                assert!(initial_latches.contains_key(latch_var));
            }
        }

        // prepare result
        let mut result = Vec::new();

        // start simulation
        for (clk_number, current_input) in inputs.iter().enumerate() {
            if clk_number == 0 {
                let last_state = self.get_initial_simulation_state(&inputs[0], initial_latches);
                result.push(last_state);
            } else {
                let last_state = result.last().unwrap();
                let next_state = self.get_next_simulation_state(last_state, current_input);
                result.push(next_state);
            }

            // println!("State {} = {:?}", clk_number, result.last().unwrap());
        }
        result
    }
}
