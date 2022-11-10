// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::AIGNodeType;
use std::collections::HashSet;

use super::AndInverterGraph;

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
    // ********************************************************************************************
    // aig getting node info
    // ********************************************************************************************

    /// Function that gets a vector describing the input nodes in the system.
    /// The output is a vector containing number representing input literals.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(Vec::<usize>::new(), aig.get_input_information());
    /// ```
    pub fn get_input_information(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for input_index in self.inputs.iter() {
            let input = &self.nodes[input_index.to_owned()];
            result.push(input.get_literal());
        }
        result
    }

    /// Function that gets a vector describing the latch nodes in the system.
    /// The output is a vector containing tuple with a length of 3,
    /// representing latch information :
    /// ```
    /// // (latch output literal, latch input literal, latch initial value)
    /// //                   ___________
    /// //                  |           |
    /// // latch input ---> |   latch   | --> latch output
    /// //                  |___________|
    /// //                        ^
    /// //                        |
    /// //               latch initial value
    /// ```
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(vec![(2, 10, 0),(4, 2, 0),(6, 4, 0)], aig.get_latch_information());
    /// ```
    pub fn get_latch_information(&self) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();
        for latch_index in self.latches.iter() {
            let latch = &self.nodes[latch_index.to_owned()];

            let latch_literal = latch.get_literal();
            let latch_input = latch.get_latch_input();
            let latch_reset = latch.get_latch_reset();

            result.push((latch_literal, latch_input, latch_reset));
        }
        result
    }

    // ********************************************************************************************
    // aig getting special literals
    // ********************************************************************************************

    /// Function that gets a vector describing the bad nodes in the system.
    /// The output is a vector containing usize numbers, these are the literals
    /// that are bad.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(Vec::<usize>::new(), aig.get_bad_information());
    /// ```
    pub fn get_bad_information(&self) -> Vec<usize> {
        self.bad.clone()
    }

    /// Function that gets a vector describing the constraints nodes in the system.
    /// The output is a vector containing usize numbers, these are the literals
    /// that are constraint.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(Vec::<usize>::new(), aig.get_constraints_information());
    /// ```
    pub fn get_constraints_information(&self) -> Vec<usize> {
        self.constraints.clone()
    }

    /// Function that gets a vector describing the output nodes in the system.
    /// The output is a vector containing usize numbers, these are the literals
    /// that are outputs of the AIG.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let mut expected_result = Vec::<usize>::new();
    /// expected_result.push(10);
    /// assert_eq!(expected_result, aig.get_output_information());
    /// ```
    pub fn get_output_information(&self) -> Vec<usize> {
        self.outputs.clone()
    }

    // ********************************************************************************************
    // aig getting numbers
    // ********************************************************************************************

    /// Function that gets the maximum variable number used in the AIG.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(5, aig.get_highest_variable_number());
    /// ```
    pub fn get_highest_variable_number(&self) -> usize {
        self.maximum_variable_index
    }

    // ********************************************************************************************
    // aig getting and gates
    // ********************************************************************************************

    /// Function that gets the cone of influence for some list of literals.
    ///
    /// # Arguments
    ///
    /// * `&self` - The AndInverterGraph desired.
    /// * `desired_literals: &[usize]` - The literals you want the AND gates for.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// // the aig looks like this:
    /// // *--------------------------------------------------------------*
    /// // |     _________                                                |
    /// // |    |         |                                               |
    /// // *--> | latch 0 |  2-*                                          |
    /// //      |_________|    |                                          |
    /// //                     |                                          |
    /// // *-------------------*-------------Not-------*                  |
    /// // |     _________                             |     ______       |
    /// // |    |         |                 ______     *--> |      \      |
    /// // *--> | latch 1 |  4-*---Not---> |      \         |  and  ) 10--*
    /// //      |_________|    |           |  and  )  8---> |______/
    /// //                     |     *---> |______/
    /// // *-------------------*     |
    /// // |     _________          Not
    /// // |    |         |          |
    /// // *--> | latch 2 |  6-------*
    /// //      |_________|
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[0]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[1]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[2]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[3]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[4]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[5]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[6]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[7]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[8]), vec![(8, 7, 5)]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[9]), vec![(8, 7, 5)]);
    /// let mut sorted = aig.get_and_information_in_cone_of_influence(&[10]);
    /// sorted.sort();
    /// assert_eq!(sorted, vec![(8, 7, 5), (10, 8, 3)]);
    /// let mut sorted = aig.get_and_information_in_cone_of_influence(&[11]);
    /// sorted.sort();
    /// assert_eq!(sorted, vec![(8, 7, 5), (10, 8, 3)]);
    /// ```
    pub fn get_and_information_in_cone_of_influence(
        &self,
        desired_literals: &[usize],
    ) -> Vec<(usize, usize, usize)> {
        let mut and_gates: HashSet<(usize, usize, usize)> = HashSet::new();
        debug_assert!(and_gates.is_empty());
        let mut current_wanted_literals = desired_literals.to_owned();
        let mut i = 0;
        while i < current_wanted_literals.len() {
            let node = &self.nodes[current_wanted_literals[i] >> 1];
            match node.get_type() {
                AIGNodeType::ConstantZero => {}
                AIGNodeType::Input => {}
                AIGNodeType::Latch => {}
                AIGNodeType::And => {
                    let and_out = node.get_literal();
                    let in0 = node.get_and_rhs0();
                    let in1 = node.get_and_rhs1();
                    if and_gates.contains(&(and_out, in0, in1)) {
                        // already visited this AND gate
                    } else {
                        // haven't seen this AND gate before
                        and_gates.insert((and_out, in0, in1));
                        current_wanted_literals.push(in0);
                        current_wanted_literals.push(in1);
                    }
                }
            };
            i += 1;
        }
        Vec::from_iter(and_gates)
    }

    // ********************************************************************************************
    // aig getting and gates
    // ********************************************************************************************
}
